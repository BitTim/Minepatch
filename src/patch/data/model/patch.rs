/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 17:21
 */

use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub name: String,
    pub pack: String,
    pub dependency: String,
}

impl Patch {
    pub(crate) fn new(name: &str, pack: &str, dependency: &str) -> Self {
        Self {
            name: name.to_owned(),
            pack: pack.to_owned(),
            dependency: dependency.to_owned(),
        }
    }
}

impl Entity for Patch {
    fn table_name() -> String {
        "patch".to_owned()
    }

    fn from_row(row: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            name: row.get(0)?,
            pack: row.get(1)?,
            dependency: row.get(2)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.pack.to_owned()),
            Box::new(self.dependency.to_owned()),
        ]
    }
}
