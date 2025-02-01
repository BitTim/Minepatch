/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   31.01.25, 02:57
 */

use crate::prelude::*;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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

    pub fn from_row(value: &Row) -> Result<Self> {
        let path: String = value.get(1)?;
        let path = PathBuf::from(path);

        Ok::<Instance, Error>(Instance {
            name: value.get(0)?,
            path,
            pack: value.get(2)?,
            patch: value.get(3)?,
        })
    }
}
