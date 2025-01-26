/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 22:00
 */
use crate::output::detailed::{DetailedDisplayObject, Entry};
use crate::output::{format_bool, format_string_option};
use colored::Colorize;
use minepatch::vault;
use minepatch::vault::Mod;
use rusqlite::Connection;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct ModListItem {
    #[tabled(rename = "Hash")]
    pub short_hash: String,
    #[tabled(rename = "Mod ID")]
    pub id: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "Loader")]
    pub loader: String,
    #[tabled(rename = "MC Version")]
    pub mc_version: String,
    #[tabled(rename = "Valid")]
    pub valid: String,
}

impl ModListItem {
    pub(crate) fn from(connection: &Connection, value: &Mod) -> Self {
        let mut short_hash = value.hash.to_owned();
        short_hash.truncate(8);

        let valid = vault::validate(connection, &value.hash);

        ModListItem {
            short_hash,
            id: value.meta.id.to_owned(),
            name: value.meta.name.to_owned(),
            version: format_string_option(&value.meta.version.to_owned()),
            loader: value.meta.loader.to_owned(),
            mc_version: format_string_option(&value.meta.minecraft_version),
            valid: format_bool(&valid),
        }
    }
}

impl DetailedDisplayObject {
    pub(crate) fn from_mod(connection: &Connection, value: &Mod) -> Self {
        let authors = value.meta.authors.as_deref().map(|value| value.join("\n"));
        let valid = vault::validate(connection, &value.hash);

        DetailedDisplayObject::new(
            vec![
                Entry::new("Hash", &value.hash.bold().purple().to_string()),
                Entry::new(
                    "Path",
                    &value.path.display().to_string().dimmed().blue().to_string(),
                ),
            ],
            vec![
                Entry::new("Mod ID", &value.meta.id.bold().yellow().to_string()),
                Entry::new("Name", &value.meta.name),
                Entry::new("Version", &format_string_option(&value.meta.version)),
                Entry::new("Authors", &format_string_option(&authors)),
                Entry::new("Loader", &value.meta.loader),
                Entry::new(
                    "Loader version",
                    &format_string_option(&value.meta.loader_version),
                ),
                Entry::new(
                    "Minecraft version",
                    &format_string_option(&value.meta.minecraft_version),
                ),
                Entry::new(
                    "Description",
                    &format_string_option(&value.meta.description),
                ),
                Entry::new("Valid", &format_bool(&valid)),
            ],
        )
    }
}
