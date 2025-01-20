/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 03:05
 */
use crate::common::meta::{fabric, forge, forge_legacy};
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub loader: String,
    pub loader_version: Option<String>,
    pub minecraft_version: Option<String>,
}

impl Meta {
    pub(crate) fn empty(filename: &str) -> Self {
        Self {
            id: "unknown".to_owned(),
            name: filename.to_owned(),
            version: None,
            description: None,
            authors: None,
            loader: "unknown".to_owned(),
            loader_version: None,
            minecraft_version: None,
        }
    }

    pub(crate) fn new(
        id: Option<String>,
        name: String,
        version: Option<String>,
        description: Option<String>,
        authors: Option<Vec<String>>,
        loader: Option<String>,
        loader_version: Option<String>,
        minecraft_version: Option<String>,
    ) -> Self {
        Self {
            id: id.unwrap_or("unknown".to_owned()),
            name,
            version,
            description,
            authors,
            loader: loader.unwrap_or("unknown".to_owned()),
            loader_version,
            minecraft_version,
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

    pub(crate) fn extract_meta(
        &self,
        data: &str,
        extra: &Option<String>,
        filename: &str,
    ) -> Result<Meta> {
        match self {
            Loader::Fabric => fabric::extract_meta(data, self.name(), filename),
            Loader::Forge | Loader::NeoForge => {
                forge::extract_meta(data, self.name(), extra, filename)
            }
            Loader::ForgeLegacy => forge_legacy::extract_meta(data, self.name(), filename),
        }
    }
}
