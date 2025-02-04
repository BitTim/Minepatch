/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:33
 */
use crate::common::{Query, QueryInsert, QueryModel};
use crate::instance::{Instance, InstanceError};
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) enum InstanceQuery {
    Insert { instance: Instance },
    QuerySimilarName { name: String },
    QueryExactName { name: String },
}

impl Query for InstanceQuery {
    fn value(&self) -> String {
        match self {
            InstanceQuery::Insert { .. } => {
                "INSERT INTO instance (name, path, pack, patch) VALUES (?1, ?2, ?3, ?4)"
            }
            InstanceQuery::QuerySimilarName { .. } => {
                "SELECT name, path, pack, patch FROM instance WHERE name LIKE ?1||'%'"
            }
            InstanceQuery::QueryExactName { .. } => {
                "SELECT name, path, pack, patch FROM instance WHERE name = ?1"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            InstanceQuery::Insert { instance } => instance.to_params(),
            InstanceQuery::QuerySimilarName { name } => vec![Box::new(name.to_owned())],
            InstanceQuery::QueryExactName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            InstanceQuery::Insert { instance } => {
                Error::Instance(InstanceError::NameTaken(instance.name.to_owned()))
            }
            InstanceQuery::QuerySimilarName { name } => {
                Error::Instance(InstanceError::NameNotFound(name.to_owned()))
            }
            InstanceQuery::QueryExactName { name } => {
                Error::Instance(InstanceError::NameNotFound(name.to_owned()))
            }
        }
    }
}

impl QueryInsert<Instance> for InstanceQuery {
    fn insert(value: Instance) -> Self {
        Self::Insert { instance: value }
    }
}
