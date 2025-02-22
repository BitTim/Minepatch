/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:00
 */
use minepatch::instance;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn link(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: &Option<String>,
    pack: &Option<String>,
) -> Result<()> {
    Ok(_ = instance::link(connection, tx, path, name.as_deref(), pack.as_deref())?)
}
