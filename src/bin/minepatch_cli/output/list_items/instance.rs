/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:52
 */
use crate::output::format_bool;
use minepatch::instance;
use minepatch::instance::Instance;
use rusqlite::Connection;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub(crate) struct InstanceListItem {
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Path")]
    pub(crate) path: String,
    #[tabled(rename = "Pack")]
    pub(crate) pack: String,
    #[tabled(rename = "Patch")]
    pub(crate) patch: String,
    #[tabled(rename = "Valid")]
    pub(crate) valid: String,
}

impl InstanceListItem {
    pub(crate) fn from(connection: &Connection, instance: &Instance) -> Self {
        let valid = instance::validate(connection, &instance.name, false);

        Self {
            name: instance.name.to_owned(),
            path: instance.path.display().to_string(),
            pack: instance.pack.to_owned(),
            patch: instance.patch.to_owned(),
            valid: format_bool(&valid),
        }
    }
}
