/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   18.02.25, 15:29
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn add(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    overwrite: &bool,
) -> Result<()> {
    Ok(_ = vault::add(conn, tx, &path::absolute(path)?.canonicalize()?, *overwrite)?)
}
