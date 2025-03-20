/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       clean.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:40
 */
use mpcore::prelude::*;

use mpcore::vault;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn clean(conn: &Connection, tx: &Sender<Event>) -> Result<()> {
    vault::clean(conn, tx)
}
