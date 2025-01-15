/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 15:45
 */

use crate::util::meta::data::Meta;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::util::{error, file};
use crate::vault::data::Mod;
use crate::vault::func::common::detect_loader::detect_loader;
use crate::vault::func::common::path::build_mod_dir_path;
use crate::vault::func::common::registry::check_entry;
use std::fs;
use std::path::Path;

pub fn add(path: &Path, silent: &bool, overwrite: &bool) -> error::Result<String> {
    let mut registry: Vec<Mod> = file::read_all()?;

    file::check_exists(path)?;
    let filename = file::filename_from_path(path)?;
    let hash = file::hash_file(path)?;
    if let Some((index, entry)) = check_entry(&*registry, &*hash).ok() {
        if *overwrite {
            registry.swap_remove(index);
        } else {
            if !silent {
                StatusOutput::new(State::Abort, "Hash is already present in vault, skipping")
                    .context("File", &*path.display().to_string())
                    .context("Vault file", &*entry.path.display().to_string())
                    .context("Hash", &hash)
                    .print();
            }

            return Ok(hash);
        }
    }

    let result = detect_loader(path)?;

    let (meta, mod_file_path) = if let Some((loader, data, extra)) = result {
        let meta = loader.extract_meta(&*data, &extra)?;
        let mod_file_path = build_mod_dir_path(&*meta.id, loader.name(), filename)?;

        (meta, mod_file_path)
    } else {
        let mod_file_path = build_mod_dir_path("unknown_id", "unknown_loader", filename)?;
        (Meta::empty(), mod_file_path)
    };

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
