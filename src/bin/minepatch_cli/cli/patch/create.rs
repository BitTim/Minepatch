/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:58
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::progress::event::Event;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    dependency: &str,
) -> minepatch::prelude::Result<()> {
    patch::create(
        connection,
        tx,
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
