/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */
use mpcore::prelude::*;
use mpcore::vault;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn remove(
    conn: &Connection,
    tx: &Sender<Event>,
    hash: &Option<String>,
    yes: bool,
    all: bool,
) -> Result<()> {
    vault::remove(conn, tx, hash.as_ref(), yes, all)
}
