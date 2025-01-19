/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 18:55
 */
use crate::util::error;
use crate::util::meta::{fabric, forge, forge_legacy};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: Option<String>,
    pub(crate) authors: Option<Vec<String>>,
    pub(crate) loader: String,
    pub(crate) loader_version: Option<String>,
    pub(crate) minecraft_version: Option<String>,
}

impl Meta {
    pub(crate) fn empty() -> Self {
        Self {
            id: "?".red().to_string(),
            name: "?".red().to_string(),
            version: "?".red().to_string(),
            description: None,
            authors: None,
            loader: "?".red().to_string(),
            loader_version: None,
            minecraft_version: None,
        }
    }
}

#[derive(EnumIter, Debug)]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
    ForgeLegacy,
}

impl Loader {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Loader::Fabric => "fabric",
            Loader::Forge => "forge",
            Loader::NeoForge => "neoforge",
            Loader::ForgeLegacy => "forge_legacy",
        }
    }

    pub(crate) fn meta_path(&self) -> &'static str {
        match self {
            Loader::Fabric => "fabric.mod.json",
            Loader::Forge => "META-INF/mods.toml",
            Loader::NeoForge => "META-INF/neoforge.mods.toml",
            Loader::ForgeLegacy => "mcmod.info",
        }
    }

    pub(crate) fn extra_path(&self) -> Option<&'static str> {
        match self {
            Loader::Fabric | Loader::ForgeLegacy => None,
            Loader::Forge | Loader::NeoForge => Some("META-INF/MANIFEST.MF"),
        }
    }

    pub(crate) fn extract_meta(&self, data: &str, extra: &Option<String>) -> error::Result<Meta> {
        match self {
            Loader::Fabric => fabric::extract_meta(data, self.name()),
            Loader::Forge | Loader::NeoForge => forge::extract_meta(data, self.name(), extra),
            Loader::ForgeLegacy => forge_legacy::extract_meta(data, self.name()),
        }
    }
}
