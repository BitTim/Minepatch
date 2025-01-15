/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 14:58
 */

use crate::util::error::ErrorType;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::util::{error, file};
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use crate::vault::func::common::registry::check_entry;
use inquire::Confirm;
use std::fs;

pub fn remove(hash: &Option<String>, all: &bool, yes: &bool, silent: &bool) -> error::Result<()> {
    let mut registry = file::read_all()?;

    let hashes: Vec<String> = if *all {
        registry
            .iter()
            .map(|entry: &Mod| entry.hash.to_owned())
            .collect()
    } else {
        match hash {
            Some(hash) => vec![hash.to_owned()],
            None => return Err(VaultError::HashNotFound.builder().build()),
        }
    };

    for hash in hashes {
        let (index, entry) = check_entry(&registry, &hash)?;
        let (name, path) = (&entry.meta.name.to_owned(), &entry.path.to_owned());

        if !yes && !confirm_remove(entry)? {
            StatusOutput::new(State::Abort, "Did not remove mod from vault")
                .context("Name", name)
                .context("Hash", &hash)
                .print();
            continue;
        }

        file::check_exists(&path)?;
        fs::remove_file(&path)?;
        file::remove_empty_dirs(&get_base_mod_dir_path()?)?;
        registry.swap_remove(index);

        if !silent {
            StatusOutput::new(State::Success, "Removed mod from vault")
                .context("Name", name)
                .context("Hash", &hash)
                .print();
        }
    }

    file::write_all(registry)?;
    Ok(())
}

pub(crate) fn confirm_remove(entry: &Mod) -> error::Result<bool> {
    let ans = Confirm::new(&format!(
        "Do you really want to remove '{}' ({}, {}) from the vault?",
        entry.meta.name, entry.meta.version, entry.meta.loader
    ))
    .with_default(false)
    .with_help_message(&format!("Hash: '{}'", &entry.hash))
    .prompt()?;

    Ok(ans)
}
