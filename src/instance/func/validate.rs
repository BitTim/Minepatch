/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 22:17
 */
use crate::db::Repo;
use crate::error::Error;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::InstanceError;
use crate::prelude::*;
use crate::{file, hash, pack, patch};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> Result<()> {
    let query = InstanceFilter::ByExactName {
        name: name.to_owned(),
    };
    let instance = InstanceRepo::query_single(connection, &query)?;

    if exist_only {
        return Ok(());
    }

    let mod_paths = file::mod_paths_from_instance_path(&instance.path)?;
    let src_dir_hash = hash::hash_state_from_path(&mod_paths)?;
    let sim_hashes = patch::simulate(connection, &instance.patch, &instance.pack)?;
    let sim_dir_hash = hash::hash_state(&sim_hashes);

    if src_dir_hash != sim_dir_hash {
        return Err(Error::Instance(InstanceError::StateMismatch {
            present_hash: src_dir_hash,
            sim_hash: sim_dir_hash,
        }));
    }

    pack::validate(connection, &instance.pack, false)?;
    patch::validate(connection, &instance.patch, &instance.pack, false)?;

    Ok(())
}
