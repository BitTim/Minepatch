/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:39
 */
use crate::output::detailed::{DetailedDisplayObject, Entry};
use crate::output::{format_bool, format_string_option};
use colored::Colorize;
use minepatch::vault::data::Mod;
use std::fs;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct ModDisplay {
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

impl TryFrom<Mod> for ModDisplay {
    type Error = ();

    fn try_from(value: Mod) -> Result<Self, Self::Error> {
        let mut short_hash = value.hash.to_owned();
        short_hash.truncate(8);

        let valid = if let Ok(value) = fs::exists(&value.path) {
            value
        } else {
            false
        };

        Ok(ModDisplay {
            short_hash,
            id: format_string_option(&value.meta.id.to_owned()),
            name: format_string_option(&value.meta.name.to_owned()),
            version: format_string_option(&value.meta.version.to_owned()),
            loader: format_string_option(&value.meta.loader.to_owned()),
            mc_version: format_string_option(&value.meta.minecraft_version),
            valid: format_bool(&valid),
        })
    }
}

impl TryFrom<Mod> for DetailedDisplayObject {
    type Error = ();

    fn try_from(value: Mod) -> Result<Self, Self::Error> {
        let authors = value.meta.authors.as_deref().map(|value| value.join("\n"));

        Ok(DetailedDisplayObject::new(
            vec![
                Entry::new("Hash", &value.hash.bold().purple().to_string()),
                Entry::new(
                    "Path",
                    &value.path.display().to_string().dimmed().blue().to_string(),
                ),
            ],
            vec![
                Entry::new(
                    "Mod ID",
                    &format_string_option(&value.meta.id.map(|id| id.bold().yellow().to_string())),
                ),
                Entry::new("Name", &format_string_option(&value.meta.name)),
                Entry::new("Version", &format_string_option(&value.meta.version)),
                Entry::new("Authors", &format_string_option(&authors)),
                Entry::new("Loader", &format_string_option(&value.meta.loader)),
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
            ],
        ))
    }
}
