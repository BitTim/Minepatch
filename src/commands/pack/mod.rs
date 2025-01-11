/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.01.25, 21:52
 */
pub mod pack_cli;
pub mod pack_main;

use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    latest_version: String,
    base: Option<Base>,
    patches: Vec<Patch>,
}

impl Pack {
    pub(crate) fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            latest_version: String::from("init"),
            base: None,
            patches: vec![Patch::empty("init")],
        }
    }

    pub(crate) fn new(name: &str, base: Option<Base>, patches: &[Patch]) -> Self {
        Self {
            name: name.to_owned(),
            latest_version: String::from("init"),
            base,
            patches: Vec::from(patches),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    name: String,
    dependency: String,
    state_hash: String,
    added: Vec<String>,
    removed: Vec<String>,
}

impl Patch {
    pub(crate) fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            dependency: String::from(""),
            state_hash: "".digest(),
            added: vec![],
            removed: vec![],
        }
    }

    pub(crate) fn new(
        name: &str,
        dependency: &str,
        state_hash: &str,
        added: &[String],
        removed: &[String],
    ) -> Self {
        Self {
            name: name.to_owned(),
            dependency: dependency.to_owned(),
            state_hash: state_hash.to_owned(),
            added: Vec::from(added),
            removed: Vec::from(removed),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    name: String,
    version: String,
    link: String,
}
