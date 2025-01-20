/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 13:47
 */
use inquire::Confirm;
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;

pub(crate) fn remove(
    connection: &Connection,
    hash: &Option<String>,
    yes: bool,
    all: bool,
) -> Result<()> {
    vault::remove(connection, hash.as_ref(), yes, all, |entry| {
        Ok(Confirm::new(&format!(
            "Do you really want to remove '{}' ({}, {}) from the vault?",
            entry.meta.name,
            entry.meta.version.clone().unwrap_or("?".to_owned()),
            entry.meta.loader
        ))
        .with_default(false)
        .with_help_message(&format!("Hash: '{}'", &entry.hash))
        .prompt()?)
    })
}
