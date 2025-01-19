/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       data.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 14:04
 */

use crate::common::meta::data::Meta;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Mod {
    pub hash: String,
    pub path: PathBuf,
    pub meta: Meta,
}

impl Mod {
    pub(crate) fn new(hash: &str, path: &Path, meta: Meta) -> Self {
        Self {
            hash: hash.to_owned(),
            path: path.to_path_buf(),
            meta,
        }
    }
}
