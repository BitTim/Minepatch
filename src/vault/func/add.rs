/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:58
 */

use crate::common::file;
use crate::common::meta::data::Meta;
use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::func::common::detect_loader::detect_loader;
use crate::vault::func::common::path::build_mod_dir_path;
use crate::vault::func::common::registry::check_entry;
use std::fs;
use std::path::Path;

pub fn add(path: &Path, overwrite: &bool) -> Result<String> {
    // TODO: Rework for SQLite
    let mut registry: Vec<Mod> = /*file::read_all()?*/ vec![];

    file::check_exists(path)?;
    let filename = file::filename_from_path(path)?;
    let hash = file::hash_file(path)?;
    if let Some((index, _)) = check_entry(&*registry, &*hash).ok() {
        if *overwrite {
            registry.swap_remove(index);
        } else {
            return Ok(hash);
        }
    }

    let result = detect_loader(path)?;

    let (meta, mod_file_path) = if let Some((loader, data, extra)) = result {
        let meta = loader.extract_meta(&*data, &extra)?;
        let mod_file_path = build_mod_dir_path(&meta.id, loader.name(), filename)?;

        (meta, mod_file_path)
    } else {
        let mod_file_path = build_mod_dir_path(&None, "unknown_loader", filename)?;
        (Meta::empty(), mod_file_path)
    };

    fs::copy(path, &mod_file_path)?;

    registry.push(Mod::new(&*hash, &mod_file_path, meta));
    //file::write_all(registry)?;
    Ok(hash)
}
