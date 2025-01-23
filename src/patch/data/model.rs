/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 02:27
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub(crate) name: String,
    pub(crate) dependency: String,
    pub(crate) state_hash: String,
    pub(crate) pack: String,
}

impl Patch {
    pub(crate) fn new(name: &str, dependency: &str, state_hash: &str, pack: &str) -> Self {
        Self {
            name: name.to_owned(),
            dependency: dependency.to_owned(),
            state_hash: state_hash.to_owned(),
            pack: pack.to_owned(),
        }
    }
}
