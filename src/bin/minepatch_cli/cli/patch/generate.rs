/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:19
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn generate(connection: &Connection, name: &str, instance: &str) -> Result<()> {
    patch::generate(connection, name, instance, &|warning| {
        StatusOutput::new(Status::Warning, Message::new(&warning.to_string())).print();
    })?;

    StatusOutput::new(
        Status::Success,
        Message::new("Generated and applied a new patch from instance")
            .context("Name", name)
            .context("Instance", instance),
    )
    .print();

    Ok(())
}
