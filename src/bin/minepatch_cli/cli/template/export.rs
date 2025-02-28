/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       export.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:34
 */
use minepatch::prelude::*;
use minepatch::template;
use rusqlite::Connection;
use std::env::current_dir;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn export(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    path: Option<&Path>,
) -> Result<()> {
    let path = match path {
        Some(path) => path.to_owned(),
        None => current_dir()?,
    };

    tx.send(Event::Log {
        message: Message::Transparent(path.display().to_string()),
    })?;

    template::export(conn, tx, name, &path)
}
