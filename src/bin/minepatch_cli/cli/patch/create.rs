/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.01.25, 19:54
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use rusqlite::Connection;

pub(crate) fn create(
    connection: &Connection,
    name: &str,
    pack: &str,
    dependency: &str,
    state_hash: &str,
) -> minepatch::prelude::Result<()> {
    patch::create(connection, name, pack, dependency, state_hash)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Added patch to pack")
            .context("Name", name)
            .context("Pack", pack),
    )
    .print();
    Ok(())
}
