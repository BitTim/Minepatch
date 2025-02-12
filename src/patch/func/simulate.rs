/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 02:32
 */
use crate::common::msg;
use crate::common::msg::Event;
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
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
) -> Result<HashSet<String>> {
    msg::init_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
    msg::tick_progress(
        tx,
        Process::Patch(PatchProcess::Simulate),
        Message::Patch(PatchMessage::SimulateStatus {
            name: name.to_owned(),
        }),
    )?;

    if name.is_empty() {
        msg::end_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
        return Ok(HashSet::new());
    }

    let patch_filter = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = PatchRepo::query_single(connection, &patch_filter)?;

    let mut mod_hashes = HashSet::new();
    mod_hashes.extend(simulate(connection, tx, &patch.dependency, pack)?);

    let rel_filter = PatchModRelFilter::ByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };
    let mod_relations = PatchModRelRepo::query_multiple(connection, &rel_filter)?;
    for relation in mod_relations {
        match relation.removed {
            true => mod_hashes.remove(&relation.mod_hash),
            false => mod_hashes.insert(relation.mod_hash.to_owned()),
        };
    }

    msg::end_progress(tx, Process::Patch(PatchProcess::Simulate), None)?;
    Ok(mod_hashes)
}

pub fn simulate_dir_hash(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
) -> Result<String> {
    let sim_hashes = simulate(connection, tx, name, pack)?;
    Ok(hash::hash_state(&sim_hashes))
}
