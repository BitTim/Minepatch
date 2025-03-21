/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       diff.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event::Event;
use crate::patch;
use crate::patch::PatchDiff;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn diff(
    conn: &Connection,
    tx: &Sender<Event>,
    bundle: &str,
    from: &str,
    to: &str,
) -> Result<PatchDiff> {
    let from_sim_hashes = patch::simulate(conn, tx, from, bundle)?;
    let to_sim_hashes = patch::simulate(conn, tx, to, bundle)?;

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
