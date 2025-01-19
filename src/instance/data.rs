/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:09
 */
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
}

impl Instance {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }
    pub(crate) fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
}
