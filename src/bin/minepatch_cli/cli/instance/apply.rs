/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:57
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::instance;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn apply(
    connection: &Connection,
    tx: &Sender<Event>,
    instance: &str,
    patch: &str,
) -> Result<()> {
    instance::apply(connection, tx, instance, patch)?;

    StatusOutput::new(Status::Success, "Applied patch to instance".to_owned())
        .context("Instance".to_owned(), instance.to_owned())
        .context("Patch".to_owned(), patch.to_owned())
        .print();
    Ok(())
}
