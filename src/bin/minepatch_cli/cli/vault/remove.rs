/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 17:07
 */
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn remove(
    connection: &Connection,
    tx: &Sender<Event>,
    hash: &Option<String>,
    yes: bool,
    all: bool,
) -> Result<()> {
    vault::remove(connection, tx, hash.as_ref(), yes, all)
}
