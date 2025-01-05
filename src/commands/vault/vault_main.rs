/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:26
 */
use crate::commands::vault::vault_util::{build_mod_dir_path, detect_loader};
use crate::commands::vault::Mod;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::Output;
use crate::common::{error, file};
use std::path::Path;

pub fn add(path: &Path) -> error::Result<()> {
    let mut registry: Vec<Mod> = file::read_all()?;

    file::check_exists(path)?;
    let filename = file::filename_from_path(path)?;
    let hash = file::hash_file(path)?;
    let (loader, data) = detect_loader(path)?;

    let meta = loader.extract_meta(&*data)?;
    let mod_file_path = build_mod_dir_path(&*meta.id, loader.name(), filename)?;
    file::move_file(path, &mod_file_path)?;

    registry.push(Mod::new(&*hash, &mod_file_path, meta));
    file::write_all(registry)?;

    StatusOutput::new(State::Success, "Registered and moved into vault")
        .context("File", &*path.display().to_string())
        .context("Vault file", &*mod_file_path.display().to_string())
        .print();
    Ok(())
}
