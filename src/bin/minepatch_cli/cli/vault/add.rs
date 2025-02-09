/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 18:31
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::event::Event;
use minepatch::msg::Message;
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

    StatusOutput::new(
        Status::Success,
        Message::new("Added mod to vault")
            .context("Hash", &hash)
            .context("Path", &path.display().to_string()),
    )
    .print();

    Ok(())
}
