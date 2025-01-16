/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:32
 */

use crate::util;
use crate::util::data::DataType;
use crate::util::error::ErrorType;
use crate::util::file::error::FileError;
use directories::ProjectDirs;
use sha256::Sha256Digest;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub mod error;
mod path_builder;
pub use path_builder::*;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

pub(crate) fn get_data_path() -> util::error::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(FileError::DataPathError.builder().build()),
        Some(project_dir) => {
            let dir = project_dir.data_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }
    }
}

pub(crate) fn init_data_file(filename: &str) -> util::error::Result<PathBuf> {
    let path = PathBuilder::new(&get_data_path()?).push(filename).build();

    if fs::exists(&path)? == false {
        fs::write(&path, "[]")?;
    }

    Ok(path)
}

fn open_data_file<T: DataType>() -> util::error::Result<File> {
    let path = init_data_file(T::FILENAME)?;
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    Ok(file)
}

pub(crate) fn filename_from_path(path: &Path) -> util::error::Result<&str> {
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

pub fn read_all<T: DataType>() -> util::error::Result<Vec<T>> {
    let file = open_data_file::<T>()?;

    let instances: Vec<T> = serde_json::from_reader(file)?;
    Ok(instances)
}

pub fn write_all<T: DataType>(mut data: Vec<T>) -> util::error::Result<()> {
    data.dedup_by(|a, b| a == b);
    let file = open_data_file::<T>()?;

    serde_json::to_writer_pretty(&file, &data)?;
    Ok(())
}

pub fn check_exists(path: &Path) -> util::error::Result<()> {
    if !fs::exists(path)? {
        Err(FileError::PathNotFound
            .builder()
            .context("Path", &path.display().to_string())
            .build())
    } else {
        Ok(())
    }
}

pub(crate) fn _move_file(path: &Path, new_path: &Path) -> util::error::Result<()> {
    fs::copy(path, new_path)?;
    fs::remove_file(path)?;

    Ok(())
}

pub(crate) fn hash_file(path: &Path) -> util::error::Result<String> {
    let mut file = fs::OpenOptions::new().read(true).open(&path)?;

    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data)?;

    Ok(data.digest())
}

pub(crate) fn remove_empty_dirs(path: &Path) -> util::error::Result<bool> {
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
