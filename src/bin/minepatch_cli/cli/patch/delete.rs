/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       delete.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 10:54
 */
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn delete(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
) -> Result<()> {
    patch::delete(conn, tx, name, bundle)
}
