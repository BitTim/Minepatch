/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:19
 */
use crate::common::{Query, QueryInsert};
use crate::error::Error;
use crate::patch::{Patch, PatchError};
use rusqlite::ToSql;

pub(crate) enum PatchQueries {
    Insert { patch: Patch },
    QueryNameAndPackExact { name: String, pack: String },
    QueryNameAndPackSimilar { name: String, pack: String },
}

impl Query for PatchQueries {
    fn value(&self) -> String {
        match self {
            PatchQueries::Insert { .. } => {
                "INSERT INTO patch (name, pack, dependency, src_dir_hash) VALUES (?1, ?2, ?3, ?4)"
            }
            PatchQueries::QueryNameAndPackExact { .. } => {
                "SELECT * FROM patch WHERE name = ?1 AND pack = ?2"
            }
            PatchQueries::QueryNameAndPackSimilar { .. } => {
                "SELECT name, dependency, src_dir_hash, pack FROM patch WHERE name LIKE ?1||'%' AND pack LIKE ?2||'%'"
            }
        }
            .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchQueries::Insert { patch } => vec![
                Box::new(patch.name.to_owned()),
                Box::new(patch.pack.to_owned()),
                Box::new(patch.dependency.to_owned()),
                Box::new(patch.src_dir_hash.to_owned()),
            ],
            PatchQueries::QueryNameAndPackExact { name, pack } => {
                vec![Box::new(name.to_owned()), Box::new(pack.to_owned())]
            }
            PatchQueries::QueryNameAndPackSimilar { name, pack } => {
                vec![Box::new(name.to_owned()), Box::new(pack.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchQueries::Insert { patch } => Error::Patch(PatchError::NameExists(
                patch.name.to_owned(),
                patch.pack.to_owned(),
            )),
            PatchQueries::QueryNameAndPackExact { name, pack }
            | PatchQueries::QueryNameAndPackSimilar { name, pack } => {
                Error::Patch(PatchError::NotFound(name.to_owned(), pack.to_owned()))
            }
        }
    }
}

impl QueryInsert<Patch> for PatchQueries {
    fn insert(value: Patch) -> Self {
        Self::Insert { patch: value }
    }
}
