/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use crate::common::event;
use crate::common::event::Event;
use crate::db::Repo;
use crate::hash;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn simulate(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
) -> Result<HashSet<String>> {
    event::init_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
    event::tick_progress(
        tx,
        Process::Patch(PatchProcess::Simulate),
        Message::Patch(PatchMessage::SimulateStatus {
            name: name.to_owned(),
        }),
    )?;

    if name.is_empty() {
        event::end_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
        return Ok(HashSet::new());
    }

    let patch_filter = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let patch = PatchRepo::query_single(conn, &patch_filter)?;

    let mut mod_hashes = HashSet::new();
    mod_hashes.extend(simulate(conn, tx, &patch.dependency, bundle)?);

    let rel_filter = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let mod_relations = PatchModRelRepo::query_multiple(conn, &rel_filter)?;
    for relation in mod_relations {
        match relation.removed {
            true => mod_hashes.remove(&relation.mod_hash),
            false => mod_hashes.insert(relation.mod_hash.to_owned()),
        };
    }

    event::end_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
    Ok(mod_hashes)
}

pub fn simulate_dir_hash(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
) -> Result<String> {
    let sim_hashes = simulate(conn, tx, name, bundle)?;
    Ok(hash::hash_state(&sim_hashes))
}
