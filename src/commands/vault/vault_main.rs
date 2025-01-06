/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 01:20
 */
use crate::commands::vault::vault_error::VaultError;
use crate::commands::vault::vault_util::{
    build_mod_dir_path, detect_loader, get_base_mod_dir_path,
};
use crate::commands::vault::Mod;
use crate::common::error::ErrorType;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::Output;
use crate::common::{error, file};
use std::fs;
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

pub fn remove(hash: &str) -> error::Result<()> {
    let mut registry: Vec<Mod> = file::read_all()?;

    let (index, entry) = registry
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.hash == hash)
        .ok_or(
            VaultError::HashNotFound
                .builder()
                .context("Hash", hash)
                .build(),
        )?;

    file::check_exists(&entry.path)?;
    fs::remove_file(&entry.path)?;
    file::remove_empty_dirs(&get_base_mod_dir_path()?)?;

    registry.remove(index);
    file::write_all(registry)?;

    StatusOutput::new(State::Success, "Removed mod from vault")
        .context("Hash", hash)
        .print();
    Ok(())
}
