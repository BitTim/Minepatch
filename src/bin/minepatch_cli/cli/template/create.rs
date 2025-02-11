/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:25
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::prelude::*;
use minepatch::template;
use rusqlite::Connection;

pub(crate) fn create(
    connection: &Connection,
    name: &str,
    loader: &Option<String>,
    version: &Option<String>,
    download: &Option<String>,
) -> Result<()> {
    template::create(
        connection,
        name,
        loader.to_owned(),
        version.to_owned(),
        download.to_owned(),
    )?;

    StatusOutput::new(
        Status::Success,
        "Added template to be used with packs".to_owned(),
    )
    .context("Name".to_owned(), name.to_owned())
    .print();
    Ok(())
}
