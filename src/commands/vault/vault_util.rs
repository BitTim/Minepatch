/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 23:55
 */
use crate::common::path_builder::PathBuilder;
use crate::common::{error, file};
use std::fs;
use std::path::PathBuf;

pub(crate) fn build_mod_dir_path(
    mod_id: &str,
    loader: &str,
    filename: &str,
) -> error::Result<PathBuf> {
    let dir = PathBuilder::new(&mut *file::get_data_path()?)
        .push("mods")
        .push(mod_id)
        .push(loader)
        .build();

    fs::create_dir_all(&dir)?;

    let path = PathBuilder::new(&dir).push(filename).build();
    Ok(path)
}
