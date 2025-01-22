/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 01:33
 */
use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn get_mod_paths(path: &Path) -> Result<Vec<PathBuf>> {
    let mod_dir_contents = fs::read_dir(&path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    let mod_paths = mod_dir_contents
        .iter()
        .filter_map(|path| {
            if path.extension().and_then(|ext| ext.to_str()) != Some("jar") {
                return None;
            }
            Some(path.to_owned())
        })
        .collect::<Vec<PathBuf>>();

    Ok(mod_paths)
}
