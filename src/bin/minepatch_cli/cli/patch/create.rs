/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:01
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    dependency: &str,
) -> Result<()> {
    patch::create(
        connection,
        tx,
        name,
        pack,
        dependency,
        &HashSet::new(),
        &HashSet::new(),
    )?;

    StatusOutput::new(Status::Success, "Added patch to pack".to_owned())
        .context("Name".to_owned(), name.to_owned())
        .context("Pack".to_owned(), pack.to_owned())
        .print();
    Ok(())
}
