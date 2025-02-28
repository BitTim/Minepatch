/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event;
use crate::common::event::Event;
use crate::hash::hash_file;
use crate::patch::{PatchMessage, PatchProcess};
use crate::prelude::*;
use crate::{file, instance, patch, vault};
use rayon::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub fn generate(conn: &Connection, tx: &Sender<Event>, name: &str, instance: &str) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Generate), None)?;

    let instance = instance::query_single(conn, instance)?;
    instance::validate(conn, tx, &instance.name, true)?;

    let sim_hashes = patch::simulate(conn, tx, &instance.patch, &instance.bundle)?;
    let mod_paths = file::mod_paths_from_instance_path(&instance.path)?;

    event::init_progress(
        tx,
        Process::Patch(PatchProcess::HashModFiles),
        Some(mod_paths.len() as u64),
    )?;
    let mod_files = mod_paths
        .par_iter()
        .map(|path| {
            let hash = hash_file(path)?;
            event::tick_progress(
                tx,
                Process::Patch(PatchProcess::HashModFiles),
                Message::Patch(PatchMessage::HashModFileStatus {
                    path: path.to_path_buf(),
                    hash: hash.to_owned(),
                }),
            )?;

            Ok((path.to_owned(), hash))
        })
        .collect::<Result<Vec<(PathBuf, String)>>>()?;
    event::end_progress(tx, Process::Patch(PatchProcess::HashModFiles), None)?;

    let mut removed = sim_hashes.to_owned();
    let mut added = HashSet::new();

    for (path, hash) in mod_files {
        removed.remove(&hash);

        if !sim_hashes.contains(&hash) {
            added.insert(vault::add(conn, tx, &path, false)?);
        }
    }

    patch::create(
        conn,
        tx,
        name,
        &instance.bundle,
        &instance.patch,
        &added,
        &removed,
    )?;

    instance::apply(conn, tx, &instance.name, name)?;
    event::end_progress(
        tx,
        Process::Patch(PatchProcess::Generate),
        Some(Message::Patch(PatchMessage::GenerateSuccess {
            name: name.to_owned(),
            instance: instance.name,
        })),
    )?;
    Ok(())
}
