/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:00
 */

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
    pub pack: String,
    pub patch: String,
}

impl Instance {
    pub fn new(name: &str, path: &Path, pack: &str, patch: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
            pack: pack.to_owned(),
            patch: patch.to_owned(),
        }
    }
}
