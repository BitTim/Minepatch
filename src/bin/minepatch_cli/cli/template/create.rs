/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:33
 */
use minepatch::prelude::*;
use minepatch::template;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    loader: &Option<String>,
    version: &Option<String>,
    download: &Option<String>,
) -> Result<()> {
    template::create(
        connection,
        tx,
        name,
        loader.to_owned(),
        version.to_owned(),
        download.to_owned(),
    )
}
