/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:40
 */
use crate::common::Query;
use crate::instance::InstanceError;
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) enum InstanceQuery {
    Insert {
        name: String,
        path: String,
        pack: String,
        patch: String,
    },
    QuerySimilarName {
        name: String,
    },
    QueryExactName {
        name: String,
    },
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

    fn params(&self) -> Vec<&(dyn ToSql + '_)> {
        match self {
            InstanceQuery::Insert {
                name,
                path,
                patch,
                pack,
            } => vec![
                name as &(dyn ToSql + '_),
                path as &(dyn ToSql + '_),
                patch as &(dyn ToSql + '_),
                pack as &(dyn ToSql + '_),
            ],
            InstanceQuery::QuerySimilarName { name } => vec![name as &(dyn ToSql + '_)],
            InstanceQuery::QueryExactName { name } => vec![name as &(dyn ToSql + '_)],
        }
    }

    fn error(&self) -> Error {
        match self {
            InstanceQuery::Insert { name, .. } => {
                Error::Instance(InstanceError::NameTaken(name.to_owned()))
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
