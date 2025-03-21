/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       detect.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event;
use crate::common::event::Event;
use crate::instance::{InstanceError, InstanceMessage, InstanceProcess};
use crate::prelude::*;
use crate::{file, hash, patch};
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn detect(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    bundle: Option<&str>,
) -> Result<(String, String)> {
    event::init_progress(tx, Process::Instance(InstanceProcess::Detect), None)?;

    let mod_paths = file::mod_paths_from_instance_path(path)?;
    let dir_hash = hash::hash_state_from_path(tx, &mod_paths)?;

    let patches = patch::query_multiple(conn, None, bundle)?;
    let mut patch_iter = patches.iter();

    let result = loop {
        let patch = patch_iter.next();
        if patch.is_none() {
            break None;
        }
        let patch = patch.unwrap();

        let sim_dir_hash = patch::simulate_dir_hash(conn, tx, &patch.name, &patch.bundle)?;
        if sim_dir_hash == dir_hash {
            break Some((patch.name.to_owned(), patch.bundle.to_owned()));
        }
    };

    let (patch, bundle) =
        result.ok_or(Error::Instance(InstanceError::NoPatchDetected { dir_hash }))?;
    event::end_progress(
        tx,
        Process::Instance(InstanceProcess::Detect),
        Some(Message::Instance(InstanceMessage::DetectSuccess {
            bundle: bundle.to_owned(),
            patch: patch.to_owned(),
        })),
    )?;

    Ok((patch, bundle))
}
