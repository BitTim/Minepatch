/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:06
 */
use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
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

impl Entity for PatchWithMods {
    fn table_name() -> String {
        "patch_with_mods".to_owned()
    }

    fn from_row(row: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            patch: row.get(0)?,
            pack: row.get(1)?,
            mod_hash: row.get(2)?,
            removed: row.get(3)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.patch.to_owned()),
            Box::new(self.pack.to_owned()),
            Box::new(self.mod_hash.to_owned()),
            Box::new(self.removed.to_owned()),
        ]
    }
}
