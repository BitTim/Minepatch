/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.01.25, 18:58
 */

use crate::common;
use crate::common::data::DataType;
use crate::common::error::{CommonError, ErrorType};
use crate::common::file::error::FileError;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub mod error;
pub mod path_builder;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

const DATA_PATH_ERROR: &str = "Did not find the projects OS specific data folder";

pub(crate) fn get_data_path() -> io::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(io::Error::new(io::ErrorKind::NotFound, DATA_PATH_ERROR)),
        Some(project_dir) => {
            let dir = project_dir.data_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }
    }
}

pub(crate) fn init_data_file(filename: &str) -> common::error::Result<PathBuf> {
    let dir = get_data_path()?;
    let path = dir.join(filename);

    if fs::exists(&path)? == false {
        fs::write(&path, "[]")?;
    }

    Ok(path)
}

pub(crate) fn filename_from_path(path: &Path) -> Result<&str, Box<dyn std::error::Error>> {
    Ok(path
        .file_name()
        .ok_or(
            FileError::PathNoFileName
                .builder()
                .context("Path", &path.display().to_string()),
        )?
        .to_str()
        .ok_or(
            FileError::PathInvalidUTF8
                .builder()
                .context("Path", &path.display().to_string()),
        )?)
}

pub fn read_all<T: DataType>() -> common::error::Result<Vec<T>> {
    let path = init_data_file(T::FILENAME)?;
    let file = fs::OpenOptions::new().read(true).open(path)?;
    let instances: Vec<T> = serde_json::from_reader(file)?;

    Ok(instances)
}

pub fn write_all<T: DataType>(data: Vec<T>) -> common::error::Result<()> {
    let path = init_data_file(T::FILENAME)?;
    let file = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(path)?;

    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

pub fn check_exists(path: &Path) -> common::error::Result<()> {
    if !fs::exists(path)? {
        Err(FileError::PathNotFound
            .builder()
            .context("Path", &path.display().to_string())
            .build())
    } else {
        Ok(())
    }
}

pub(crate) fn move_file(path: &Path, new_path: &Path) -> common::error::Result<()> {
    fs::copy(path, new_path)?;
    fs::remove_file(path)?;

    Ok(())
}

pub(crate) fn hash_file(path: &Path) -> common::error::Result<String> {
    match sha256::try_digest(path) {
        Ok(value) => Ok(value),
        Err(error) => Err(CommonError::Wrapper(Box::new(error))
            .builder()
            .context("Cause", "File hashing")
            .context("File", &path.display().to_string())
            .build()),
    }
}

pub(crate) fn remove_empty_dirs(path: &Path) -> common::error::Result<bool> {
    if path.is_dir() {
        let mut is_empty = true;
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                let sub_path = entry.path();
                if !remove_empty_dirs(&sub_path)? {
                    is_empty = false;
                }
            }
        }

        if is_empty {
            fs::remove_dir(path)?
        }
        return Ok(is_empty);
    }

    Ok(false)
}
