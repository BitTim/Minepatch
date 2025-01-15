/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 12:04
 */

use crate::util::data::DataType;
use crate::util::error::{CommonError, ErrorType};
use crate::util::file::error::FileError;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::vault;
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

pub(crate) fn init_data_file(filename: &str) -> crate::util::error::Result<PathBuf> {
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

pub fn read_all<T: DataType>() -> crate::util::error::Result<Vec<T>> {
    let path = init_data_file(T::FILENAME)?;
    let file = fs::OpenOptions::new().read(true).open(path)?;
    let instances: Vec<T> = serde_json::from_reader(file)?;

    Ok(instances)
}

pub fn write_all<T: DataType>(data: Vec<T>) -> crate::util::error::Result<()> {
    let path = init_data_file(T::FILENAME)?;
    let file = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(path)?;

    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

pub fn check_exists(path: &Path) -> crate::util::error::Result<()> {
    if !fs::exists(path)? {
        Err(FileError::PathNotFound
            .builder()
            .context("Path", &path.display().to_string())
            .build())
    } else {
        Ok(())
    }
}

pub(crate) fn _move_file(path: &Path, new_path: &Path) -> crate::util::error::Result<()> {
    fs::copy(path, new_path)?;
    fs::remove_file(path)?;

    Ok(())
}

pub(crate) fn hash_file(path: &Path) -> crate::util::error::Result<String> {
    match sha256::try_digest(path) {
        Ok(value) => Ok(value),
        Err(error) => Err(CommonError::Wrapper(Box::new(error))
            .builder()
            .context("Cause", "File hashing")
            .context("File", &path.display().to_string())
            .build()),
    }
}

pub(crate) fn remove_empty_dirs(path: &Path) -> crate::util::error::Result<bool> {
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

pub(crate) fn extract_hashes_from_dir(
    path: &Path,
    silent: bool,
) -> crate::util::error::Result<Vec<String>> {
    let mod_files = fs::read_dir(&path)?;
    Ok(mod_files
        .map(|mod_file| {
            let path = mod_file?.path();
            if !silent {
                println!("Found file '{}'", path.display());
            }

            // Adds mod file to vault and returns the hash
            vault::func::add::add(&path, &true)
        })
        .filter_map(|result| match result {
            Ok(hash) => {
                if !silent {
                    println!("Generated file hash: '{}'\n", hash);
                }
                Some(hash)
            }
            Err(error) => {
                if !silent {
                    StatusOutput::new(State::Error, "Error occurred during mod file hashing")
                        .context("Message", &*error.to_string())
                        .context("Path", &path.display().to_string())
                        .print();
                }
                None
            }
        })
        .collect::<Vec<String>>())
}
