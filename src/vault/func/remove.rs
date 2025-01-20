/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 03:09
 */

use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use inquire::Confirm;

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
        //let (index, entry) = check_exists(&registry, &hash)?;
        //let path = &entry.path.to_owned();

        //if !yes && !confirm_remove(entry)? {
        //    continue;
        //}

        //file::check_exists(&path)?;
        //fs::remove_file(&path)?;
        //file::remove_empty_dirs(&get_base_mod_dir_path()?)?;
        //registry.swap_remove(index);
    }

    //file::write_all(registry)?;
    Ok(())
}

pub(crate) fn confirm_remove(entry: &Mod) -> Result<bool> {
    let ans = Confirm::new(&format!(
        "Do you really want to remove '{}' ({}, {}) from the vault?",
        entry.meta.name,
        entry.meta.version.clone().unwrap_or("?".to_owned()),
        entry.meta.loader
    ))
    .with_default(false)
    .with_help_message(&format!("Hash: '{}'", &entry.hash))
    .prompt()?;

    Ok(ans)
}
