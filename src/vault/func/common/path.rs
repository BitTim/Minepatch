/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   07.02.25, 17:34
 */

use crate::common::file;
use crate::prelude::*;
use std::fs;
use std::path::PathBuf;

pub(crate) fn get_base_mod_dir_path() -> Result<PathBuf> {
    let mut dir = file::get_data_path()?;
    dir.push("mods");

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub(crate) fn build_mod_dir_path(mod_id: &str, loader: &str, filename: &str) -> Result<PathBuf> {
    let mut dir = get_base_mod_dir_path()?;
    dir.push(mod_id);
    dir.push(loader);

    fs::create_dir_all(&dir)?;

    let mut path = dir;
    path.push(filename);
    Ok(path)
}
