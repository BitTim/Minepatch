/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 22:29
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::prelude::*;
use minepatch::progress::event::Event;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn generate(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    instance: &str,
) -> Result<()> {
    patch::generate(connection, tx, name, instance)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Generated and applied a new patch from instance")
            .context("Name", name)
            .context("Instance", instance),
    )
    .print();

    Ok(())
}
