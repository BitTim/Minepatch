/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:32
 */
use minepatch::instance;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn apply(
    connection: &Connection,
    tx: &Sender<Event>,
    instance: &str,
    patch: &str,
) -> Result<()> {
    instance::apply(connection, tx, instance, patch)
}
