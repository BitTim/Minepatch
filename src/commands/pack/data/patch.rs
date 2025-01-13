/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 20:29
 */
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

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
