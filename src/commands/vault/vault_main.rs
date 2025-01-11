/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.01.25, 19:01
 */
use crate::commands::vault::vault_error::VaultError;
use crate::commands::vault::vault_util::{
    build_mod_dir_path, check_entry, confirm_remove, detect_loader, filter_registry,
    get_base_mod_dir_path,
};
use crate::commands::vault::{Mod, ModDisplay};
use crate::common::error::ErrorType;
use crate::common::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::table::TableOutput;
use crate::common::output::Output;
use crate::common::{error, file};
use std::fs;
use std::path::Path;
use tabled::settings::object::Columns;

pub fn add(path: &Path) -> error::Result<()> {
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

    StatusOutput::new(State::Success, "Registered and moved into vault")
        .context("File", &*path.display().to_string())
        .context("Vault file", &*mod_file_path.display().to_string())
        .print();
    Ok(())
}

pub fn list(detailed: &bool, hash: &Option<String>, id: &Option<String>) -> error::Result<()> {
    let registry: Vec<Mod> = file::read_all()?;
    let filtered = filter_registry(&registry, hash, id);

    let mod_displays = filtered
        .iter()
        .map(|entry| entry.to_display())
        .collect::<Vec<ModDisplay>>();

    match detailed {
        false => {
            TableOutput::new(mod_displays)
                .right(Columns::new(3..4))
                .right(Columns::new(5..6))
                .center(Columns::new(6..7))
                .print();
        }
        true => {
            DetailedOutput::new(
                filtered
                    .iter()
                    .map(|&entry| entry.to_detailed_display())
                    .collect::<Vec<DetailedDisplayObject>>(),
            )
            .print();
        }
    }

    Ok(())
}

pub fn remove(hash: &Option<String>, all: &bool, yes: &bool) -> error::Result<()> {
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
        registry.remove(index);

        StatusOutput::new(State::Success, "Removed mod from vault")
            .context("Name", name)
            .context("Hash", &hash)
            .print();
    }

    file::write_all(registry)?;
    Ok(())
}
