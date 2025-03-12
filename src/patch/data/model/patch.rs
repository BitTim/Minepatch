/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       patch.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:13
 */

use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub name: String,
    pub bundle: String,
    pub dependency: String,
}

impl Patch {
    pub(crate) fn new(name: &str, bundle: &str, dependency: &str) -> Self {
        Self {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
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
            bundle: row.get(1)?,
            dependency: row.get(2)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.bundle.to_owned()),
            Box::new(self.dependency.to_owned()),
        ]
    }
}
