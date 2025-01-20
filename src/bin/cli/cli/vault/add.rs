/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 01:36
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path::Path;

pub(crate) fn add(connection: &Connection, path: &Path, overwrite: &bool) -> Result<String> {
    Ok(vault::add(connection, path, *overwrite)?)
}
