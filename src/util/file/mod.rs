/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 16:32
 */

use crate::util::error::ErrorType;
use crate::util::file::error::FileError;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::{env, io};

pub mod error;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

const DATA_PATH_ERROR: &str = "Did not find the projects OS specific data folder";

pub(crate) fn get_data_path() -> io::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(io::Error::new(io::ErrorKind::NotFound, DATA_PATH_ERROR)),
        Some(project_dir) => Ok(project_dir.data_dir().to_path_buf()),
    }
}

pub(crate) fn get_filename(path: &Path) -> Result<&str, Box<dyn std::error::Error>> {
    let context = Some(format!(
        "Path: '{}'",
        path.to_path_buf().display().to_string()
    ));

    Ok(path
        .file_name()
        .ok_or(FileError::PathNoFileName.with_context(context.clone()))?
        .to_str()
        .ok_or(FileError::PathInvalidUTF8.with_context(context.clone()))?)
}
