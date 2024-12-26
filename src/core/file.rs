/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       file.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 03:05
 */
use directories::ProjectDirs;
use std::path::PathBuf;
use std::{env, io};

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

const DATA_PATH_ERROR_MSG: &str = "Did not find the projects OS specific data folder";

pub(crate) fn get_data_path() -> io::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(io::Error::new(io::ErrorKind::NotFound, DATA_PATH_ERROR_MSG)),
        Some(project_dir) => Ok(project_dir.data_dir().to_path_buf()),
    }
}
