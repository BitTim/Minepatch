/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:40
 */
use mpcore::prelude::*;

use mpcore::vault;
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
