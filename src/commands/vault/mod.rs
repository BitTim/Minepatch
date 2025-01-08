/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.01.25, 16:18
 */
use crate::commands::vault::meta::Meta;
use crate::common::output::detailed::{DetailedDisplayObject, Entry};
use crate::common::output::{format_bool, format_string_option};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tabled::Tabled;

pub mod meta;
pub mod vault_cli;
mod vault_error;
pub mod vault_main;
mod vault_util;

#[derive(Debug, Tabled)]
pub struct ModDisplay {
    #[tabled(rename = "Hash")]
    short_hash: String,
    #[tabled(rename = "Mod ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Version")]
    version: String,
    #[tabled(rename = "Loader")]
    loader: String,
    #[tabled(rename = "MC Version")]
    mc_version: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    hash: String,
    path: PathBuf,
    meta: Meta,
}

impl Mod {
    fn new(hash: &str, path: &Path, meta: Meta) -> Self {
        Self {
            hash: hash.to_owned(),
            path: path.to_path_buf(),
            meta,
        }
    }

    fn to_display(&self) -> ModDisplay {
        let mut short_hash = self.hash.to_owned();
        short_hash.truncate(8);

        let valid = if let Ok(value) = fs::exists(&self.path) {
            value
        } else {
            false
        };

        ModDisplay {
            short_hash,
            id: self.meta.id.to_owned(),
            name: self.meta.name.to_owned(),
            version: self.meta.version.to_owned(),
            loader: self.meta.loader.to_owned(),
            mc_version: format_string_option(&self.meta.minecraft_version),
            valid: format_bool(&valid),
        }
    }

    fn to_detailed_display(&self) -> DetailedDisplayObject {
        let authors = &self.meta.authors.join("\n");

        DetailedDisplayObject::new(
            vec![
                Entry::new("Hash", &self.hash.bold().purple().to_string()),
                Entry::new(
                    "Path",
                    &self.path.display().to_string().dimmed().blue().to_string(),
                ),
            ],
            vec![
                Entry::new("Mod ID", &self.meta.id.bold().yellow().to_string()),
                Entry::new("Name", &self.meta.name),
                Entry::new("Version", &self.meta.version),
                Entry::new("Authors", authors),
                Entry::new("Loader", &self.meta.loader),
                Entry::new(
                    "Loader version",
                    &format_string_option(&self.meta.loader_version),
                ),
                Entry::new(
                    "Minecraft version",
                    &format_string_option(&self.meta.minecraft_version),
                ),
                Entry::new("Description", &self.meta.description),
            ],
        )
    }
}
