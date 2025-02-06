/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::patch;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn exclude(
    connection: &Connection,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    patch::exclude(connection, name, pack, mod_hash)?;

    StatusOutput::new(
        Status::Success,
        Message::new("Excluded mod with patch")
            .context("Mod", mod_hash)
            .context("Patch", name)
            .context("Pack", pack),
    )
    .print();
    Ok(())
}
