/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       rename.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:37
 */

use mpcore::patch;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn rename(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    new_name: &str,
) -> Result<()> {
    patch::rename(conn, tx, name, bundle, new_name)
}
