/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:24
 */
use crate::common::QueryModel;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: Option<String>,
    pub loader: Option<String>,
    pub download: Option<String>,
}

impl Template {
    pub(crate) fn new(
        name: &str,
        version: Option<String>,
        loader: Option<String>,
        download: Option<String>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            version,
            loader,
            download,
        }
    }
}

impl QueryModel for Template {
    fn from_row(row: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            name: row.get(0)?,
            version: row.get(1)?,
            loader: row.get(2)?,
            download: row.get(3)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.version.to_owned()),
            Box::new(self.loader.to_owned()),
            Box::new(self.download.to_owned()),
        ]
    }
}
