/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 13:42
 */

use crate::common::file::error::FileError;
use crate::prelude::*;
use directories::ProjectDirs;
use sha256::Sha256Digest;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub mod error;
mod path_builder;
pub use path_builder::*;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

pub(crate) fn get_data_path() -> Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(Error::File(FileError::DataPathError)),
        Some(project_dir) => {
            let dir = project_dir.data_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }
    }
}

pub(crate) fn filename_from_path(path: &Path) -> Result<&str> {
    path.file_name()
        .ok_or(Error::File(FileError::PathNoFileName(
            path.display().to_string(),
        )))?
        .to_str()
        .ok_or(Error::File(FileError::PathInvalidUTF8(
            path.display().to_string(),
        )))
}

pub fn check_exists(path: &Path) -> Result<()> {
    if !fs::exists(path)? {
        Err(Error::File(FileError::PathNotFound(
            path.display().to_string(),
        )))
    } else {
        Ok(())
    }
}

pub(crate) fn _move_file(path: &Path, new_path: &Path) -> Result<()> {
    fs::copy(path, new_path)?;
    fs::remove_file(path)?;

    Ok(())
}

pub(crate) fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::OpenOptions::new().read(true).open(&path)?;

    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data)?;

    Ok(data.digest())
}

pub(crate) fn remove_empty_dirs(path: &Path) -> Result<bool> {
    if path.is_dir() {
        let mut is_empty = true;
        for entry in (fs::read_dir(path)?).flatten() {
            let sub_path = entry.path();
            if !remove_empty_dirs(&sub_path)? {
                is_empty = false;
            }
        }

        if is_empty {
            fs::remove_dir(path)?
        }
        return Ok(is_empty);
    }

    Ok(false)
}
