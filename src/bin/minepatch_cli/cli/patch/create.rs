/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:33
 */
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    dependency: &str,
) -> Result<()> {
    patch::create(
        connection,
        tx,
        name,
        pack,
        dependency,
        &HashSet::new(),
        &HashSet::new(),
    )
}
