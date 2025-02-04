/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:11
 */
use crate::common::{Query, QueryInsert, QueryModel};
use crate::error::Error;
use crate::pack::{Pack, PackError};
use rusqlite::ToSql;

pub(crate) enum PackQueries {
    Insert { pack: Pack },
    QueryExactName { name: String },
    QuerySimilarName { name: String },
}

impl Query for PackQueries {
    fn value(&self) -> String {
        match self {
            PackQueries::Insert { .. } => {
                "INSERT INTO pack (name, description, template) VALUES (?1, ?2, ?3)"
            }
            PackQueries::QueryExactName { .. } => "SELECT * FROM pack WHERE name = ?1",
            PackQueries::QuerySimilarName { .. } => {
                "SELECT name, description, template FROM pack WHERE name LIKE ?1||'%'"
            }
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PackQueries::Insert { pack } => pack.to_params(),
            PackQueries::QueryExactName { name } => vec![Box::new(name.to_owned())],
            PackQueries::QuerySimilarName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            PackQueries::Insert { pack } => Error::Pack(PackError::NameTaken(pack.name.to_owned())),
            PackQueries::QueryExactName { name } | PackQueries::QuerySimilarName { name } => {
                Error::Pack(PackError::NotFound(name.to_owned()))
            }
        }
    }
}

impl QueryInsert<Pack> for PackQueries {
    fn insert(value: Pack) -> Self {
        Self::Insert { pack: value }
    }
}
