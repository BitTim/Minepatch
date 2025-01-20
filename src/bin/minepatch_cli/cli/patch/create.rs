/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:20
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use rusqlite::Connection;

pub(crate) fn create(
    connection: &Connection,
    name: &str,
    dependency: &str,
    state_hash: &str,
    pack: &str,
) -> minepatch::prelude::Result<()> {
    patch::create(connection, name, dependency, state_hash, pack)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Added template to be used with packs").context("Name", name),
    )
    .print();
    Ok(())
}
