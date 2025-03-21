/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:17
 */
use mpcore::bundle;
use mpcore::event::Event;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn import(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: Option<&str>,
) -> Result<()> {
    bundle::import(conn, tx, path, name)
}
