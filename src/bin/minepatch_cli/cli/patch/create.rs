/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 02:11
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use rusqlite::Connection;
use std::collections::HashSet;

pub(crate) fn create(
    connection: &Connection,
    name: &str,
    pack: &str,
    dependency: &str,
) -> minepatch::prelude::Result<()> {
    patch::create(
        connection,
        name,
        pack,
        dependency,
        &HashSet::new(),
        &HashSet::new(),
    )?;

    StatusOutput::new(
        Status::Success,
        Message::new("Added patch to pack")
            .context("Name", name)
            .context("Pack", pack),
    )
    .print();
    Ok(())
}
