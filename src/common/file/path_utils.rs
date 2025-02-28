/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path_utils.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::db::Entity;
use crate::file::error::FileError;
use crate::file::file_utils::check_exists;
use crate::file::{ORGANIZATION, QUALIFIER};
use crate::prelude::*;
use directories::ProjectDirs;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{fs, path};

pub fn get_data_path() -> Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(FileError::DataPathError.into()),
        Some(project_dir) => {
            let dir = project_dir.data_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }
    }
}

pub fn filename_from_path(path: &Path) -> Result<&str> {
    path.file_name()
        .ok_or(Error::File(FileError::PathNoFileName {
            path: path.to_owned(),
        }))?
        .to_str()
        .ok_or(Error::File(FileError::PathInvalidUTF8 {
            path: path.to_owned(),
        }))
}

pub fn mod_paths_from_instance_path(path: &Path) -> Result<Vec<PathBuf>> {
    let mut path = path.to_owned();
    path.push("mods");

    check_exists(&path)?;
    let mod_dir_contents = fs::read_dir(&path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    let mod_paths = mod_dir_contents
        .iter()
        .filter_map(|path| {
            if path.extension().and_then(|ext| ext.to_str()) != Some("jar") {
                return None;
            }
            Some(path.to_owned())
        })
        .collect::<Vec<PathBuf>>();

    Ok(mod_paths)
}

pub(crate) fn canonicalize_entity_path<T>(mut path: PathBuf, entity_name: &str) -> Result<PathBuf>
where
    T: Entity,
{
    let extension = path.extension().map(ToOwned::to_owned);

    if let Some(ext) = extension {
        if ext != OsString::from(T::file_extension()) {
            return Err(Error::File(FileError::InvalidExtension {
                path: path.clone(),
                expected: T::file_extension(),
                extension: ext
                    .to_str()
                    .ok_or(Error::File(FileError::PathInvalidUTF8 { path }))?
                    .to_owned(),
            }));
        }

        let filename = filename_from_path(&path)?.to_owned();
        path.pop();

        let mut canonicalized_parent = path::absolute(path)?.canonicalize()?;
        canonicalized_parent.push(filename);

        Ok(canonicalized_parent)
    } else {
        let mut canonicalized_dir = path::absolute(path)?.canonicalize()?;
        canonicalized_dir.push(format!("{}.{}", entity_name, T::file_extension()));
        Ok(canonicalized_dir)
    }
}
