/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:35
 */
use mpcore::instance;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::path;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn link(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: &Option<String>,
    bundle: &Option<String>,
) -> Result<()> {
    Ok(_ = instance::link(
        conn,
        tx,
        &path::absolute(path)?.canonicalize()?,
        name.as_deref(),
        bundle.as_deref(),
    )?)
}
