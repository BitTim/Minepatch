/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.01.25, 20:56
 */
pub mod pack_cli;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    latest_version: String,
    mc_version: String,
    loader: String,
    patches: Vec<Patch>,
}

impl Pack {
    pub(crate) fn new(name: &str, mc_version: &str, loader: &str, patches: &[Patch]) -> Self {
        Self {
            name: name.to_owned(),
            latest_version: String::from("init"),
            mc_version: mc_version.to_owned(),
            loader: loader.to_owned(),
            patches: Vec::from(patches),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    name: String,
    dependency: String,
    hashed_state: String,
    added: Vec<String>,
    removed: Vec<String>,
}

impl Patch {
    pub(crate) fn new(
        name: &str,
        dependency: &str,
        hashed_state: &str,
        added: &[String],
        removed: &[String],
    ) -> Self {
        Self {
            name: name.to_owned(),
            dependency: dependency.to_owned(),
            hashed_state: hashed_state.to_owned(),
            added: Vec::from(added),
            removed: Vec::from(removed),
        }
    }
}
