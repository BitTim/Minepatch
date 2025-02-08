/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 22:17
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::instance;
use minepatch::msg::Message;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn apply(connection: &Connection, instance: &str, patch: &str) -> Result<()> {
    instance::apply(connection, instance, patch)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Applied patch to instance")
            .context("Instance", instance)
            .context("Patch", patch),
    )
    .print();
    Ok(())
}
