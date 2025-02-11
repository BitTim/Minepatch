/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:19
 */
use crate::common::msg::Event;
use crate::hash::hash_file;
use crate::prelude::*;
use crate::{file, instance, patch, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub fn generate(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    instance: &str,
) -> Result<()> {
    let instance = instance::query_single(connection, instance)?;
    instance::validate(connection, tx, &instance.name, true)?;

    let sim_hashes = patch::simulate(connection, tx, &instance.patch, &instance.pack)?;
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
            added.insert(vault::add(connection, tx, &path, false)?);
        }
    }

    patch::create(
        connection,
        tx,
        name,
        &instance.pack,
        &instance.patch,
        &added,
        &removed,
    )?;

    instance::apply(connection, tx, &instance.name, name)?;
    Ok(())
}
