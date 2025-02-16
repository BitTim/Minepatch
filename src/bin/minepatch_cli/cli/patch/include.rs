/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:33
 */
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn include(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::include(connection, tx, name, pack, mod_hash)
}
