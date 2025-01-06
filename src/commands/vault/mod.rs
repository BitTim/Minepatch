/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 18:17
 */
use crate::commands::vault::meta::Meta;
use crate::common::output::{format_bool, format_string_option};
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
}
