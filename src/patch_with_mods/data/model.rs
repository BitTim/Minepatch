/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:31
 */
use crate::common::QueryModel;
use crate::prelude::*;
use rusqlite::{Row, ToSql};

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

impl QueryModel for PatchWithMods {
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
