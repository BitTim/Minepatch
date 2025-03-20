/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:36
 */

use mpcore::patch;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn include(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::include(conn, tx, name, bundle, mod_hash)
}
