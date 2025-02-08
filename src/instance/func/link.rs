/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 22:14
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::hash;
use crate::db::Repo;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{validate, Instance, InstanceError};
use crate::prelude::*;
use crate::{file, instance, pack, patch};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub fn link(
    connection: &Connection,
    path: &Path,
    name: &Option<String>,
    pack: &str,
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

    let query = InstanceFilter::ByExactName {
        name: actual_name.to_owned(),
    };
    if InstanceRepo::exists(connection, &query)? {
        return Err(Error::Instance(InstanceError::NameTaken {
            name: actual_name.to_owned(),
        }));
    }

    pack::validate(connection, pack, true)?;

    let mod_paths = file::mod_paths_from_instance_path(path)?;
    let src_dir_hash = hash::hash_state_from_path(&mod_paths)?;
    let patch = patch::query_by_src_dir_hash_single(connection, &src_dir_hash, pack)?;
    let dependency = &patch.dependency;

    if dependency.is_empty() {
        return Err(Error::Instance(InstanceError::NoPatchFound {
            pack: pack.to_owned(),
            src_dir_hash,
        }));
    }

    InstanceRepo::insert(
        connection,
        Instance::new(actual_name, path, pack, dependency),
    )?;
    validate(connection, actual_name, false)?;

    instance::apply(connection, actual_name, dependency)?;
    Ok(actual_name.to_owned())
}
