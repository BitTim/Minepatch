/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::output::format_bool;
use minepatch::instance;
use minepatch::instance::Instance;
use minepatch::prelude::Event;
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub(crate) struct InstanceListItem {
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Path")]
    pub(crate) path: String,
    #[tabled(rename = "Bundle")]
    pub(crate) bundle: String,
    #[tabled(rename = "Patch")]
    pub(crate) patch: String,
    #[tabled(rename = "Valid")]
    pub(crate) valid: String,
}

impl InstanceListItem {
    pub(crate) fn from(conn: &Connection, tx: &Sender<Event>, instance: &Instance) -> Self {
        let valid = instance::validate(conn, tx, &instance.name, false).is_ok();

        Self {
            name: instance.name.to_owned(),
            path: instance.path.display().to_string(),
            bundle: instance.bundle.to_owned(),
            patch: instance.patch.to_owned(),
            valid: format_bool(&valid),
        }
    }
}
