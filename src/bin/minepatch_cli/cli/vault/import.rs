/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 00:17
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn import(conn: &Connection, tx: &Sender<Event>, path: &Path) -> Result<()> {
    vault::import(conn, tx, path)
}
