/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:52
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::instance;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::progress::event::Event;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn apply(
    connection: &Connection,
    tx: &Sender<Event>,
    instance: &str,
    patch: &str,
) -> Result<()> {
    instance::apply(connection, tx, instance, patch)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Applied patch to instance")
            .context("Instance", instance)
            .context("Patch", patch),
    )
    .print();
    Ok(())
}
