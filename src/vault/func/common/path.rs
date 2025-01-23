/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 03:06
 */

use crate::common::file;
use crate::common::file::PathBuilder;
use crate::prelude::*;
use std::fs;
use std::path::PathBuf;

pub(crate) fn get_base_mod_dir_path() -> Result<PathBuf> {
    let dir = PathBuilder::new(&file::get_data_path()?)
        .push("mods")
        .build();

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub(crate) fn build_mod_dir_path(mod_id: &str, loader: &str, filename: &str) -> Result<PathBuf> {
    let dir = PathBuilder::new(&get_base_mod_dir_path()?)
        .push(mod_id)
        .push(loader)
        .build();

    fs::create_dir_all(&dir)?;

    let path = PathBuilder::new(&dir).push(filename).build();
    Ok(path)
}
