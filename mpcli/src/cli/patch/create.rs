/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:36
 */

use mpcore::patch;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    dependency: &str,
) -> Result<()> {
    patch::create(
        conn,
        tx,
        name,
        bundle,
        dependency,
        &HashSet::new(),
        &HashSet::new(),
    )
}
