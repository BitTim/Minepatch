/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 17:59
 */
use directories::ProjectDirs;
use std::env;
use std::path::PathBuf;

pub(crate) fn get_data_path() -> Result<PathBuf, &'static str> {
    match ProjectDirs::from("dev", "BitTim", env!("CARGO_PKG_NAME")) {
        None => Err("Could not find home directory"),
        Some(project_dir) => Ok(project_dir.data_dir().to_path_buf()),
    }
}
