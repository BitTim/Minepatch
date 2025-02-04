/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:16
 */
use crate::common::QueryModel;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Pack {
    pub name: String,
    pub description: Option<String>,
    pub template: Option<String>,
}

impl Pack {
    pub fn new(name: &str, description: Option<String>, template: Option<String>) -> Self {
        Self {
            name: name.to_owned(),
            description,
            template,
        }
    }
}

impl QueryModel for Pack {
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
