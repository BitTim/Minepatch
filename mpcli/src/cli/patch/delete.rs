/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       delete.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:36
 */

use mpcore::patch;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn delete(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
) -> Result<()> {
    patch::delete(conn, tx, name, bundle)
}
