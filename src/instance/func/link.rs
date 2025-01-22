/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 19:08
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::instance::{data, Instance, InstanceError};
use crate::pack::PackError;
use crate::patch::PatchError;
use crate::prelude::*;
use crate::{file, pack, patch};
use rusqlite::Connection;
use sha256::Sha256Digest;
use std::fs;
use std::path::Path;

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

    if data::exists(connection, actual_name)? {
        return Err(Error::Instance(InstanceError::NameTaken(
            actual_name.to_owned(),
        )));
    }

    if !pack::data::exists(connection, pack)? {
        return Err(Error::Pack(PackError::NotFound(pack.to_owned())));
    }

    if !patch::data::exists(connection, patch, pack)? {
        return Err(Error::Patch(PatchError::NotFound(
            patch.to_owned(),
            pack.to_owned(),
        )));
    }

    data::insert(connection, Instance::new(actual_name, path, pack, patch))?;

    // 1. [x] Query list of mods
    // 2. [x] Generate state hash
    // 3. [ ] Simulate patches up until applied
    // 4. [ ] Generate state hash of simulation
    // 5. [ ] Check state hash
    // 6. [ ] Delete mod file
    // 7. [ ] Create Symlink

    let mod_paths = file::mod_paths_from_instance_path(path)?;
    let mut hashes = vec![];
    for mod_path in mod_paths {
        hashes.push(file::hash_file(&mod_path)?);
    }

    let state_hash = hashes.join("\n").digest();

    Ok(actual_name.to_owned())
}
