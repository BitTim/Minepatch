/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path_utils.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 07:38
 */
use crate::db::Portable;
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

pub(crate) fn canonicalize_entity_path<'a, T>(mut path: PathBuf, entity: &T) -> Result<PathBuf>
where
    T: Portable,
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
        canonicalized_dir.push(format!("{}.{}", entity.object_name(), T::file_extension()));
        Ok(canonicalized_dir)
    }
}

pub(crate) fn get_base_vault_path() -> Result<PathBuf> {
    let mut dir = get_data_path()?;
    dir.push("mods");

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub(crate) fn build_vault_path(mod_id: &str, loader: &str, filename: &str) -> Result<PathBuf> {
    let mut dir = get_base_vault_path()?;
    dir.push(mod_id);
    dir.push(loader);

    fs::create_dir_all(&dir)?;

    let mut path = dir;
    path.push(filename);
    Ok(path)
}
