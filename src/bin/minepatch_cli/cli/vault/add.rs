/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 02:11
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::vault;
use rusqlite::Connection;
use std::path::Path;

pub(crate) fn add(connection: &Connection, path: &Path, overwrite: &bool) -> Result<()> {
    let hash = vault::add(connection, path, *overwrite, &|warning| {
        StatusOutput::new(Status::Warning, Message::new(&warning.to_string())).print();
    })?;

    StatusOutput::new(
        Status::Success,
        Message::new("Added mod to vault")
            .context("Hash", &hash)
            .context("Path", &path.display().to_string()),
    )
    .print();

    Ok(())
}
