/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       diff.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:50
 */
use crate::patch;
use crate::patch::PatchDiff;
use crate::prelude::*;
use crate::progress::event::Event;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn diff(
    connection: &Connection,
    tx: &Sender<Event>,
    pack: &str,
    from: &str,
    to: &str,
) -> Result<PatchDiff> {
    let from_sim_hashes = patch::simulate(connection, tx, from, pack)?;
    let to_sim_hashes = patch::simulate(connection, tx, to, pack)?;

    let added = to_sim_hashes
        .difference(&from_sim_hashes)
        .map(|value| value.to_owned())
        .collect::<HashSet<String>>();

    let removed = from_sim_hashes
        .difference(&to_sim_hashes)
        .map(|value| value.to_owned())
        .collect::<HashSet<String>>();

    Ok(PatchDiff::new(from, to, &added, &removed))
}
