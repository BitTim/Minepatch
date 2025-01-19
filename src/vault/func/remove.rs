/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:54
 */

use crate::common::file;
use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use crate::vault::func::common::registry::check_entry;
use inquire::Confirm;
use std::fs;

pub fn remove(hash: &Option<String>, all: &bool, yes: &bool) -> Result<()> {
    // TODO: Rework for SQLite
    let mut registry = /*file::read_all()?*/ vec![];

    let hashes: Vec<String> = if *all {
        registry
            .iter()
            .map(|entry: &Mod| entry.hash.to_owned())
            .collect()
    } else {
        match hash {
            Some(hash) => vec![hash.to_owned()],
            None => return Err(Error::Vault(VaultError::HashNotFound("".to_owned()))),
        }
    };

    for hash in hashes {
        let (index, entry) = check_entry(&registry, &hash)?;
        let path = &entry.path.to_owned();

        if !yes && !confirm_remove(entry)? {
            continue;
        }

        file::check_exists(&path)?;
        fs::remove_file(&path)?;
        file::remove_empty_dirs(&get_base_mod_dir_path()?)?;
        registry.swap_remove(index);
    }

    //file::write_all(registry)?;
    Ok(())
}

pub(crate) fn confirm_remove(entry: &Mod) -> Result<bool> {
    let ans = Confirm::new(&format!(
        "Do you really want to remove '{}' ({}, {}) from the vault?",
        entry.meta.name.clone().unwrap_or("?".to_owned()),
        entry.meta.version.clone().unwrap_or("?".to_owned()),
        entry.meta.loader.clone().unwrap_or("?".to_owned())
    ))
    .with_default(false)
    .with_help_message(&format!("Hash: '{}'", &entry.hash))
    .prompt()?;

    Ok(ans)
}
