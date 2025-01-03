/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   03.01.25, 23:55
 */
mod fabric_based;
mod forge_based;
mod meta_error;

use crate::common::error;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    id: String,
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    loader: String,
    loader_version: Option<String>,
    minecraft_version: Option<String>,
}

#[derive(EnumIter, Debug)]
pub enum Loader {
    NeoForge,
    Forge,
    Fabric,
    Quilt,
}

impl Loader {
    fn name(&self) -> &'static str {
        match self {
            Loader::Forge => "Forge",
            Loader::Fabric => "Fabric",
            Loader::NeoForge => "NeoForge",
            Loader::Quilt => "Quilt",
        }
    }

    pub(crate) fn meta_path(&self) -> &'static str {
        match self {
            Loader::Forge => "META-INF/mods.toml",
            Loader::Fabric => "fabric.mod.json",
            Loader::NeoForge => "META-INF/neoforge.mods.toml",
            Loader::Quilt => "quilt.mod.json",
        }
    }

    pub(crate) fn extract_meta(&self, data: &str) -> error::Result<Meta> {
        match self {
            Loader::Forge | Loader::NeoForge => forge_based::extract_meta(data, self.name()),
            Loader::Fabric | Loader::Quilt => fabric_based::extract_meta(data, self.name()),
        }
    }
}
