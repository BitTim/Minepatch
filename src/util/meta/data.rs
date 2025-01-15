/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */
use crate::util::error;
use crate::util::meta::{fabric, forge};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: String,
    pub(crate) authors: Vec<String>,
    pub(crate) loader: String,
    pub(crate) loader_version: Option<String>,
    pub(crate) minecraft_version: Option<String>,
}

#[derive(EnumIter, Debug)]
pub enum Loader {
    NeoForge,
    Forge,
    Fabric,
}

impl Loader {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Loader::Forge => "forge",
            Loader::Fabric => "fabric",
            Loader::NeoForge => "neoforge",
        }
    }

    pub(crate) fn meta_path(&self) -> &'static str {
        match self {
            Loader::Forge => "META-INF/mods.toml",
            Loader::Fabric => "fabric.mod.json",
            Loader::NeoForge => "META-INF/neoforge.mods.toml",
        }
    }

    pub(crate) fn extra_path(&self) -> Option<&'static str> {
        match self {
            Loader::Forge | Loader::NeoForge => Some("META-INF/MANIFEST.MF"),
            Loader::Fabric => None,
        }
    }

    pub(crate) fn extract_meta(&self, data: &str, extra: &Option<String>) -> error::Result<Meta> {
        match self {
            Loader::Forge | Loader::NeoForge => forge::extract_meta(data, self.name(), extra),
            Loader::Fabric => fabric::extract_meta(data, self.name()),
        }
    }
}
