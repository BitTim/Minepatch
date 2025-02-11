/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:00
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn add(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    overwrite: &bool,
) -> Result<()> {
    let hash = vault::add(connection, tx, path, *overwrite)?;

    StatusOutput::new(Status::Success, "Added mod to vault".to_owned())
        .context("Hash".to_owned(), hash)
        .context("Path".to_owned(), path.display().to_string())
        .print();

    Ok(())
}
