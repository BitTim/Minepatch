/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       generate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:25
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::patch;
use minepatch::prelude::*;
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
        "Generated and applied a new patch from instance".to_owned(),
    )
    .context("Name".to_owned(), name.to_owned())
    .context("Instance".to_owned(), instance.to_owned())
    .print();

    Ok(())
}
