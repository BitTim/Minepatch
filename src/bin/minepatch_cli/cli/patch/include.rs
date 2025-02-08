/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.02.25, 18:11
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn include(
    connection: &Connection,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::include(connection, name, pack, mod_hash)?;

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
