/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:50
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::pack;
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
        name,
        description.as_deref(),
        template.as_deref(),
        from.as_deref(),
        instance.as_deref(),
    )?;

    StatusOutput::new(Status::Success, "Created new pack".to_owned())
        .context("Name".to_owned(), name.to_owned())
        .print();
    Ok(())
}
