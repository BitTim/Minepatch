/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 23:55
 */
use crate::commands::vault::meta::Meta;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub mod meta;
pub mod vault_cli;
mod vault_error;
pub mod vault_main;
mod vault_util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    hash: String,
    path: PathBuf,
    meta: Meta,
}

impl Mod {
    fn new(hash: &str, path: &Path, meta: Meta) -> Self {
        Self {
            hash: hash.to_owned(),
            path: path.to_path_buf(),
            meta,
        }
    }
}
