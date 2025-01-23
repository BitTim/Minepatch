/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 18:54
 */

pub struct PatchWithMods {
    pub patch: String,
    pub pack: String,
    pub mod_hash: String,
    pub removed: bool,
}

impl PatchWithMods {
    pub fn new(patch: &str, pack: &str, mod_hash: &str, removed: bool) -> Self {
        Self {
            patch: patch.to_owned(),
            pack: pack.to_owned(),
            mod_hash: mod_hash.to_owned(),
            removed,
        }
    }
}
