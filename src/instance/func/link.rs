/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:19
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::hash;
use crate::db::Repo;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{validate, Instance, InstanceError};
use crate::prelude::*;
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

    let query = InstanceFilter::QueryExactName {
        name: actual_name.to_owned(),
    };
    if InstanceRepo::exists(connection, &query)? {
        return Err(Error::Instance(InstanceError::NameTaken(
            actual_name.to_owned(),
        )));
    }

    pack::validate(connection, pack, true)?;
    patch::validate(connection, patch, pack, true)?;

    InstanceRepo::insert(connection, Instance::new(actual_name, path, pack, patch))?;
    validate(connection, &actual_name, false)?;

    let mod_paths = file::mod_paths_from_instance_path(path)?;

    // TODO: Create all symlinks first and only remove all jar files if all symlinks are created
    for mod_path in mod_paths {
        let hash = hash::hash_file(&mod_path)?;
        let mod_entry = vault::query_single(connection, &hash)?;

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
