/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:32
 */
use crate::common::Query;
use crate::instance::InstanceError;
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) enum InstanceQuery {
    GeneralFilter { name: String },
    ByName { name: String },
}

impl Query for InstanceQuery {
    fn value(&self) -> String {
        match self {
            InstanceQuery::GeneralFilter { .. } => {
                "SELECT name, path, pack, patch FROM instance WHERE name LIKE ?1||'%'"
            }
            InstanceQuery::ByName { .. } => {
                "SELECT name, path, pack, patch FROM instance WHERE name = ?1"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<&(dyn ToSql + '_)> {
        match self {
            InstanceQuery::GeneralFilter { name } => vec![name as &(dyn ToSql + '_)],
            InstanceQuery::ByName { name } => vec![name as &(dyn ToSql + '_)],
        }
    }

    fn error(&self) -> Error {
        match self {
            InstanceQuery::GeneralFilter { name } => {
                Error::Instance(InstanceError::NameNotFound(name.to_owned()))
            }
            InstanceQuery::ByName { name } => {
                Error::Instance(InstanceError::NameNotFound(name.to_owned()))
            }
        }
    }
}
