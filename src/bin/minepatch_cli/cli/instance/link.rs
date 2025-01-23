/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 19:10
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::instance;
use minepatch::msg::Message;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::path::Path;

pub(crate) fn link(
    connection: &Connection,
    path: &Path,
    name: &Option<String>,
    pack: &str,
    patch: &str,
) -> Result<()> {
    let name = instance::link(connection, path, name, pack, patch)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Linked instance to pack with specified patch")
            .context("Name", &name)
            .context("Path", &path.display().to_string())
            .context("Pack", pack)
            .context("Patch", patch),
    )
    .print();

    Ok(())
}
