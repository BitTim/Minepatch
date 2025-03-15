/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       clean.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 14:16
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn clean(conn: &Connection, tx: &Sender<Event>) -> Result<()> {
    vault::clean(conn, tx)
}
