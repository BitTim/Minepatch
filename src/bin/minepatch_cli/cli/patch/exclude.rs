/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:00
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn exclude(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::exclude(connection, tx, name, pack, mod_hash)?;

    StatusOutput::new(Status::Success, "Excluded mod with patch".to_owned())
        .context("Mod".to_owned(), mod_hash.to_owned())
        .context("Patch".to_owned(), name.to_owned())
        .context("Pack".to_owned(), pack.to_owned())
        .print();
    Ok(())
}
