/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:57
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use crate::pack::{Pack, PackError};
use rusqlite::ToSql;

pub(crate) enum PackFilter {
    Insert { pack: Pack },
    QueryExactName { name: String },
    QuerySimilarName { name: String },
}

impl Filter for PackFilter {
    fn value(&self) -> String {
        match self {
            PackFilter::Insert { .. } => "VALUES (?1, ?2, ?3)",
            PackFilter::QueryExactName { .. } => "WHERE name = ?1",
            PackFilter::QuerySimilarName { .. } => "WHERE name LIKE ?1||'%'",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PackFilter::Insert { pack } => pack.to_params(),
            PackFilter::QueryExactName { name } => vec![Box::new(name.to_owned())],
            PackFilter::QuerySimilarName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            PackFilter::Insert { pack } => Error::Pack(PackError::NameTaken(pack.name.to_owned())),
            PackFilter::QueryExactName { name } | PackFilter::QuerySimilarName { name } => {
                Error::Pack(PackError::NotFound(name.to_owned()))
            }
        }
    }
}

impl InsertableFilter<Pack> for PackFilter {
    fn insert(value: Pack) -> Self {
        Self::Insert { pack: value }
    }
}
