/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:36
 */

use mpcore::patch;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn generate(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    instance: &str,
) -> Result<()> {
    patch::generate(conn, tx, name, instance)
}
