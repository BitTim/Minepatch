/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch_diff.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 16:50
 */
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PatchDiff {
    pub from: String,
    pub to: String,

    pub added: HashSet<String>,
    pub removed: HashSet<String>,
}

impl PatchDiff {
    pub fn new(from: &str, to: &str, added: &HashSet<String>, removed: &HashSet<String>) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            added: added.to_owned(),
            removed: removed.to_owned(),
        }
    }
}
