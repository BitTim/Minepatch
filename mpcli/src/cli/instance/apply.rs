/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:17
 */
use mpcore::instance;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn apply(
    conn: &Connection,
    tx: &Sender<Event>,
    instance: &str,
    patch: &str,
) -> Result<()> {
    instance::apply(conn, tx, instance, patch)
}
