/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:58
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::prelude::*;
use minepatch::progress::event::Event;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn include(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::include(connection, tx, name, pack, mod_hash)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Included mod with patch")
            .context("Mod", mod_hash)
            .context("Patch", name)
            .context("Pack", pack),
    )
    .print();
    Ok(())
}
