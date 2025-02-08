/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 02:38
 */
use crate::hash::hash_file;
use crate::prelude::*;
use crate::{file, instance, patch, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::PathBuf;

pub fn generate<F>(
    connection: &Connection,
    name: &str,
    instance: &str,
    handle_warning: &F,
) -> Result<()>
where
    F: Fn(Error),
{
    let instance = instance::query_single(connection, instance)?;
    instance::validate(connection, &instance.name, true)?;

    let sim_hashes = patch::simulate(connection, &instance.patch, &instance.pack)?;
    let mod_paths = file::mod_paths_from_instance_path(&instance.path)?;
    let mod_files = mod_paths
        .iter()
        .map(|path| Ok((path.clone(), hash_file(path)?)))
        .collect::<Result<Vec<(PathBuf, String)>>>()?;

    let mut removed = sim_hashes.clone();
    let mut added = HashSet::new();

    for (path, hash) in mod_files {
        removed.remove(&hash);

        if !sim_hashes.contains(&hash) {
            added.insert(vault::add(connection, &path, false, handle_warning)?);
        }
    }

    patch::create(
        connection,
        name,
        &instance.pack,
        &instance.patch,
        &added,
        &removed,
    )?;

    // TODO: Add apply function
    //instance::apply(connection, &instance.name, name)?;

    Ok(())
}
