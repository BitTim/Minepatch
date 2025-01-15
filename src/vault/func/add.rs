/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */

use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::util::{error, file};
use crate::vault::data::Mod;
use crate::vault::func::common::detect_loader::detect_loader;
use crate::vault::func::common::path::build_mod_dir_path;
use std::fs;
use std::path::Path;

pub fn add(path: &Path, silent: &bool) -> error::Result<String> {
    let mut registry: Vec<Mod> = file::read_all()?;

    file::check_exists(path)?;
    let filename = file::filename_from_path(path)?;
    let hash = file::hash_file(path)?;
    let (loader, data, extra) = detect_loader(path)?;

    let meta = loader.extract_meta(&*data, &extra)?;
    let mod_file_path = build_mod_dir_path(&*meta.id, loader.name(), filename)?;
    fs::copy(path, &mod_file_path)?;

    registry.push(Mod::new(&*hash, &mod_file_path, meta));
    file::write_all(registry)?;

    if !silent {
        StatusOutput::new(State::Success, "Registered and moved into vault")
            .context("File", &*path.display().to_string())
            .context("Vault file", &*mod_file_path.display().to_string())
            .context("Hash", &hash)
            .print();
    }

    Ok(hash)
}
