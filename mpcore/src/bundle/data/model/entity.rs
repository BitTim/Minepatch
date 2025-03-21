/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       entity.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 11:29
 */
use crate::common::db::Entity;
use crate::prelude::*;
use bincode::{Decode, Encode};
use rusqlite::{Row, ToSql};

/// An object that represents a bundle of patches.
///
/// This object implements [Entity] and can be used in a database.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Encode, Decode)]
pub struct Bundle {
    pub name: String,
    pub description: Option<String>,
}

impl Bundle {
    /// Constructor
    ///
    /// Creates a new instance of [Bundle] from the parameters
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            description: description.map(|description| description.to_owned()),
        }
    }
}

impl Entity for Bundle {
    /// Returns the name of the table for bundles
    fn table_name() -> String {
        "bundle".to_owned()
    }

    /// Creates a new instance of [Bundle] from a database row
    fn from_row(value: &Row) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            name: value.get(0)?,
            description: value.get(1)?,
        }))
    }

    /// Creates a list of values from a [Bundle] instance
    fn to_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.to_owned()),
            Box::new(self.description.to_owned()),
        ]
    }
}
