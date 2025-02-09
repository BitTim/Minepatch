/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 18:30
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::event::Event;
use minepatch::msg::Message;
use minepatch::pack;
use minepatch::pack::Pack;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    description: &Option<String>,
    template: &Option<String>,
    from: &Option<String>,
    instance: &Option<String>,
) -> Result<()> {
    pack::create(
        connection,
        tx,
        Pack::new(name, description.to_owned(), template.to_owned()),
        from.to_owned(),
        instance.to_owned(),
    )?;

    StatusOutput::new(
        Status::Success,
        Message::new("Created new pack").context("Name", name),
    )
    .print();
    Ok(())
}
