/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:12
 */
use crate::output::format_bool;
use minepatch::patch;
use minepatch::patch::Patch;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use tabled::Tabled;

#[derive(Tabled, Debug)]
pub struct PatchListItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Pack")]
    pack: String,
    #[tabled(rename = "Dependency")]
    dependency: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

impl PatchListItem {
    pub(crate) fn from(connection: &Connection, tx: &Sender<Event>, value: &Patch) -> Result<Self> {
        let valid = patch::validate(connection, tx, &value.name, &value.pack, false).is_ok();

        Ok(PatchListItem {
            name: value.name.to_owned(),
            pack: value.pack.to_owned(),
            dependency: value.dependency.to_owned(),
            valid: format_bool(&valid),
        })
    }
}
