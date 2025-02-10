/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       detect.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 19:07
 */
use crate::instance::InstanceError;
use crate::prelude::*;
use crate::progress::event::Event;
use crate::{file, hash, patch, progress};
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn detect(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    pack: Option<&str>,
) -> Result<(String, String)> {
    let id = progress::init_progress(tx, "Detecting patch and pack", None)?;

    let mod_paths = file::mod_paths_from_instance_path(path)?;
    let dir_hash = hash::hash_state_from_path(tx, &mod_paths)?;

    let patches = patch::query_multiple(connection, None, pack)?;
    let mut patch_iter = patches.iter();

    let result = loop {
        let patch = patch_iter.next();
        if patch.is_none() {
            break None;
        }
        let patch = patch.unwrap();

        let sim_dir_hash = patch::simulate_dir_hash(connection, tx, &patch.name, &patch.pack)?;
        if sim_dir_hash == dir_hash {
            break Some((patch.name.to_owned(), patch.pack.to_owned()));
        }
    };

    progress::end_progress(tx, id)?;
    Ok(result.ok_or(Error::Instance(InstanceError::NoPatchDetected { dir_hash }))?)
}
