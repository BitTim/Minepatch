/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       model.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:43
 */

use crate::common::db::Entity;
use crate::prelude::*;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
    pub bundle: String,
    pub patch: String,
}

impl Instance {
    pub fn new(name: &str, path: &Path, bundle: &str, patch: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
            bundle: bundle.to_owned(),
            patch: patch.to_owned(),
        }
    }
}

impl Entity for Instance {
    fn table_name() -> String {
        "instance".to_owned()
    }
    fn file_extension() -> String {
        "mpi".to_owned()
    }

    fn from_row(value: &Row) -> Result<Box<Self>> {
        let path: String = value.get(1)?;
        let path = PathBuf::from(path);

        Ok(Box::new(Self {
            name: value.get(0)?,
            path,
            bundle: value.get(2)?,
            patch: value.get(3)?,
        }))
    }

    fn to_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.path.display().to_string()),
            Box::new(self.bundle.to_owned()),
            Box::new(self.patch.to_owned()),
        ]
    }
}
