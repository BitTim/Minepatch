/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
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
    #[tabled(rename = "Bundle")]
    bundle: String,
    #[tabled(rename = "Dependency")]
    dependency: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

impl PatchListItem {
    pub(crate) fn from(conn: &Connection, tx: &Sender<Event>, value: &Patch) -> Result<Self> {
        let valid = patch::validate(conn, tx, &value.name, &value.bundle, false).is_ok();

        Ok(PatchListItem {
            name: value.name.to_owned(),
            bundle: value.bundle.to_owned(),
            dependency: value.dependency.to_owned(),
            valid: format_bool(&valid),
        })
    }
}
