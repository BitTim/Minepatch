/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.01.25, 19:59
 */
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub name: String,
    pub pack: String,
    pub dependency: String,
    pub src_dir_hash: String,
}

impl Patch {
    pub(crate) fn new(name: &str, pack: &str, dependency: &str, src_dir_hash: &str) -> Self {
        Self {
            name: name.to_owned(),
            pack: pack.to_owned(),
            dependency: dependency.to_owned(),
            src_dir_hash: src_dir_hash.to_owned(),
        }
    }
}
