/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:33
 */
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn generate(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    instance: &str,
) -> Result<()> {
    patch::generate(connection, tx, name, instance)
}
