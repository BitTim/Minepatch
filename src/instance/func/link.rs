/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:33
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::{hash, Repo};
use crate::instance::data::{InstanceQuery, InstanceRepo};
use crate::instance::{Instance, InstanceError};
use crate::pack::PackError;
use crate::patch::PatchError;
use crate::prelude::*;
use crate::vault::VaultError;
use crate::{file, pack, patch, vault};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};

pub fn link(
    connection: &Connection,
    path: &Path,
    name: &Option<String>,
    pack: &str,
    patch: &str,
) -> Result<String> {
    if !fs::exists(path)? {
        return Err(Error::File(FileError::PathNotFound(
            path.display().to_string(),
        )));
    }

    let actual_name = match name {
        Some(name) => name,
        None => filename_from_path(path)?,
    };

    if InstanceRepo::exists(
        connection,
        InstanceQuery::QueryExactName {
            name: actual_name.to_owned(),
        },
    )? {
        return Err(Error::Instance(InstanceError::NameTaken(
            actual_name.to_owned(),
        )));
    }

    if !pack::validate(connection, pack, true) {
        return Err(Error::Pack(PackError::NotFound(pack.to_owned())));
    }

    if !patch::validate(connection, patch, pack, true) {
        return Err(Error::Patch(PatchError::NotFound(
            patch.to_owned(),
            pack.to_owned(),
        )));
    }

    InstanceRepo::insert(connection, Instance::new(actual_name, path, pack, patch))?;

    let mod_paths = file::mod_paths_from_instance_path(path)?;
    let hashes: Result<Vec<String>> = mod_paths
        .iter()
        .map(|mod_path| hash::hash_file(mod_path))
        .collect();

    let mut hashes = hashes?;
    hashes.sort();
    let present_state_hash = hash::hash_state(&hashes);

    let mut simulated_hashes = patch::simulate(connection, patch, pack)?;
    simulated_hashes.sort();
    let simulated_state_hash = hash::hash_state(&simulated_hashes);

    if present_state_hash != simulated_state_hash {
        return Err(Error::Instance(InstanceError::StateMismatch(
            present_state_hash,
            simulated_state_hash,
        )));
    }

    // TODO: Create all symlinks first and only remove all jar files if all symlinks are created
    for mod_path in mod_paths {
        let hash = hash::hash_file(&mod_path)?;
        let mod_entries = vault::query(connection, Some(&hash), None, None)?;
        let mod_entry = mod_entries
            .first()
            .ok_or(Error::Vault(VaultError::NotFound(hash)))?;

        if !fs::exists(&mod_entry.path)? {
            return Err(Error::File(FileError::PathNotFound(
                mod_entry.path.display().to_string(),
            )));
        }

        let tmp_mod_path = PathBuf::from(mod_path.display().to_string() + ".tmp");
        fs::rename(&mod_path, &tmp_mod_path)?;
        symlink::symlink_file(&mod_entry.path, &mod_path)?;

        if !fs::exists(&mod_path)? {
            fs::rename(&tmp_mod_path, &mod_path)?;
            return Err(Error::Instance(InstanceError::SymlinkFailed(
                mod_entry.path.display().to_string(),
                mod_path.display().to_string(),
            )));
        }

        fs::remove_file(&tmp_mod_path)?;
    }

    Ok(actual_name.to_owned())
}
