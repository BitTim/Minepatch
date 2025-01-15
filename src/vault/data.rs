/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 15:20
 */

use crate::util::meta::data::Meta;
use crate::util::output::detailed::{DetailedDisplayObject, Entry};
use crate::util::output::{format_bool, format_string_option};
use crate::vault::output::ModDisplay;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub(crate) hash: String,
    pub(crate) path: PathBuf,
    pub(crate) meta: Meta,
}

impl Mod {
    pub(crate) fn new(hash: &str, path: &Path, meta: Meta) -> Self {
        Self {
            hash: hash.to_owned(),
            path: path.to_path_buf(),
            meta,
        }
    }

    pub(crate) fn to_display(&self) -> ModDisplay {
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

    pub(crate) fn to_detailed_display(&self) -> DetailedDisplayObject {
        let authors = self.meta.authors.as_deref().map(|value| value.join("\n"));

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
                Entry::new("Authors", &format_string_option(&authors)),
                Entry::new("Loader", &self.meta.loader),
                Entry::new(
                    "Loader version",
                    &format_string_option(&self.meta.loader_version),
                ),
                Entry::new(
                    "Minecraft version",
                    &format_string_option(&self.meta.minecraft_version),
                ),
                Entry::new("Description", &format_string_option(&self.meta.description)),
            ],
        )
    }
}
