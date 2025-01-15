/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 18:55
 */
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
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
