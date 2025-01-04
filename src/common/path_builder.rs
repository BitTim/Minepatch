/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       path_builder.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 21:09
 */
use std::path::{Path, PathBuf};

pub struct PathBuilder {
    path: PathBuf,
}

impl PathBuilder {
    pub(crate) fn new(base_path: &Path) -> Self {
        Self {
            path: PathBuf::from(base_path),
        }
    }

    pub(crate) fn push(&mut self, path: &str) -> &mut Self {
        self.path.push(path);
        self
    }

    pub(crate) fn build(&mut self) -> PathBuf {
        self.path.to_owned()
    }
}
