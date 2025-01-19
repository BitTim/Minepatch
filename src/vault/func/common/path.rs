/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:32
 */

use crate::util::file::PathBuilder;
use crate::util::{error, file};
use std::fs;
use std::path::PathBuf;

pub(crate) fn get_base_mod_dir_path() -> error::Result<PathBuf> {
    let dir = PathBuilder::new(&mut *file::get_data_path()?)
        .push("mods")
        .build();

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub(crate) fn build_mod_dir_path(
    mod_id: &str,
    loader: &str,
    filename: &str,
) -> error::Result<PathBuf> {
    let dir = PathBuilder::new(&mut *get_base_mod_dir_path()?)
        .push(mod_id)
        .push(loader)
        .build();

    fs::create_dir_all(&dir)?;

    let path = PathBuilder::new(&dir).push(filename).build();
    Ok(path)
}
