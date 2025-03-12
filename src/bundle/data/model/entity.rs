/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       entity.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 17:12
 */

use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub name: String,
    pub description: Option<String>,
    pub template: Option<String>,
}

impl Bundle {
    pub fn new(name: &str, description: Option<&str>, template: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            description: description.map(|description| description.to_owned()),
            template: template.map(|template| template.to_owned()),
        }
    }
}

impl Entity for Bundle {
    fn table_name() -> String {
        "bundle".to_owned()
    }

    fn from_row(value: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            name: value.get(0)?,
            description: value.get(1)?,
            template: value.get(2)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.description.to_owned()),
            Box::new(self.template.to_owned()),
        ]
    }
}
