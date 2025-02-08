/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:05
 */
use crate::common::db::Entity;
use crate::common::meta::data::Meta;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Mod {
    pub hash: String,
    pub path: PathBuf,
    pub meta: Meta,
}

impl Mod {
    pub(crate) fn new(hash: &str, path: &Path, meta: Meta) -> Self {
        Self {
            hash: hash.to_owned(),
            path: path.to_path_buf(),
            meta,
        }
    }
}

impl Entity for Mod {
    fn table_name() -> String {
        "mod".to_owned()
    }

    fn from_row(row: &Row) -> Result<Box<Self>> {
        let path: String = row.get(1)?;
        let path = PathBuf::from(path);

        let authors: Option<String> = row.get(6)?;
        let authors = authors.map(|value| {
            value
                .split(",")
                .map(|author| author.to_owned())
                .collect::<Vec<String>>()
        });

        Ok(Box::new(Self {
            hash: row.get(0)?,
            path,
            meta: Meta {
                id: row.get(2)?,
                name: row.get(3)?,
                version: row.get(4)?,
                description: row.get(5)?,
                authors,
                loader: row.get(7)?,
                loader_version: row.get(8)?,
                minecraft_version: row.get(9)?,
            },
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        let authors = &self.meta.authors;
        let authors = authors.clone().map(|value| value.join(","));

        vec![
            Box::new(self.hash.to_owned()),
            Box::new(self.path.display().to_string()),
            Box::new(self.meta.id.to_owned()),
            Box::new(self.meta.name.to_owned()),
            Box::new(self.meta.version.to_owned()),
            Box::new(self.meta.description.to_owned()),
            Box::new(authors),
            Box::new(self.meta.loader.to_owned()),
            Box::new(self.meta.loader_version.to_owned()),
            Box::new(self.meta.minecraft_version.to_owned()),
        ]
    }
}
