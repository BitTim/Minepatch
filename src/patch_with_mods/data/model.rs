/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct PatchModRelation {
    pub patch: String,
    pub bundle: String,
    pub mod_hash: String,
    pub removed: bool,
}

impl PatchModRelation {
    pub fn new(patch: &str, bundle: &str, mod_hash: &str, removed: bool) -> Self {
        Self {
            patch: patch.to_owned(),
            bundle: bundle.to_owned(),
            mod_hash: mod_hash.to_owned(),
            removed,
        }
    }
}

impl Entity for PatchModRelation {
    fn table_name() -> String {
        "patch_with_mods".to_owned()
    }

    fn from_row(row: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            patch: row.get(0)?,
            bundle: row.get(1)?,
            mod_hash: row.get(2)?,
            removed: row.get(3)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.patch.to_owned()),
            Box::new(self.bundle.to_owned()),
            Box::new(self.mod_hash.to_owned()),
            Box::new(self.removed.to_owned()),
        ]
    }
}
