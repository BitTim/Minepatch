/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path_builder.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 21:25
 */
use std::path::{Path, PathBuf};

pub struct PathBuilder {
    path: PathBuf,
}

impl PathBuilder {
    pub fn new(base_path: &Path) -> Self {
        Self {
            path: PathBuf::from(base_path),
        }
    }

    pub fn push(mut self, path: &str) -> Self {
        self.path.push(path);
        self
    }

    pub fn build(self) -> PathBuf {
        self.path
    }
}
