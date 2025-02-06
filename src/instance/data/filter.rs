/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:56
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::instance::{Instance, InstanceError};
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) enum InstanceFilter {
    Insert { instance: Instance },
    QuerySimilarName { name: String },
    QueryExactName { name: String },
}

impl Filter for InstanceFilter {
    fn value(&self) -> String {
        match self {
            InstanceFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            InstanceFilter::QuerySimilarName { .. } => "WHERE name LIKE ?1||'%'",
            InstanceFilter::QueryExactName { .. } => "WHERE name = ?1",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            InstanceFilter::Insert { instance } => instance.to_params(),
            InstanceFilter::QuerySimilarName { name } => vec![Box::new(name.to_owned())],
            InstanceFilter::QueryExactName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            InstanceFilter::Insert { instance } => {
                Error::Instance(InstanceError::NameTaken(instance.name.to_owned()))
            }
            InstanceFilter::QuerySimilarName { name } | InstanceFilter::QueryExactName { name } => {
                Error::Instance(InstanceError::NameNotFound(name.to_owned()))
            }
        }
    }
}

impl InsertableFilter<Instance> for InstanceFilter {
    fn insert(value: Instance) -> Self {
        Self::Insert { instance: value }
    }
}
