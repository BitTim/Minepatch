/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       export.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 23:53
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::env::current_dir;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn export(
    conn: &Connection,
    tx: &Sender<Event>,
    hash: &str,
    path: Option<&Path>,
) -> Result<()> {
    let path = match path {
        None => &current_dir()?,
        Some(path) => path,
    };

    vault::export(conn, tx, hash, path)
}
