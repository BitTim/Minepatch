/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:34
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn add(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    overwrite: &bool,
) -> Result<()> {
    Ok(_ = vault::add(connection, tx, path, *overwrite)?)
}
