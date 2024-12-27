/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       file.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 02:41
 */
use crate::error;
use crate::error::path::PathError;
use directories::ProjectDirs;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, io};

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

const DATA_PATH_ERROR: &str = "Did not find the projects OS specific data folder";

pub(crate) fn get_data_path() -> io::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(io::Error::new(io::ErrorKind::NotFound, DATA_PATH_ERROR)),
        Some(project_dir) => Ok(project_dir.data_dir().to_path_buf()),
    }
}

pub(crate) fn get_filename(path: &Path) -> Result<&str, Box<dyn Error>> {
    let context = Some(path.to_path_buf().display().to_string());

    Ok(path
        .file_name()
        .ok_or(error::Error::new(
            Box::new(PathError::PathNoFileName),
            context.clone(),
        ))?
        .to_str()
        .ok_or(error::Error::new(
            Box::new(PathError::PathInvalidUTF8),
            context,
        ))?)
}
