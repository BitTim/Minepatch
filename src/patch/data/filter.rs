/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:08
 */
use crate::common::db::{Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::{Patch, PatchError};
use rusqlite::ToSql;

pub(crate) enum PatchFilter {
    Insert { patch: Patch },
    ByNameAndPackExact { name: String, pack: String },
    QueryByNameAndPackSimilar { name: String, pack: String },
    QueryByDepAndPackExact { dependency: String, pack: String },
}

impl Filter for PatchFilter {
    fn value(&self) -> String {
        match self {
            PatchFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            PatchFilter::ByNameAndPackExact { .. } => "WHERE name = ?1 AND pack = ?2",
            PatchFilter::QueryByNameAndPackSimilar { .. } => {
                "WHERE name LIKE ?1||'%' AND pack LIKE ?2||'%'"
            }
            PatchFilter::QueryByDepAndPackExact { .. } => "WHERE dependency = ?1 AND pack = ?2",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchFilter::Insert { patch } => vec![
                Box::new(patch.name.to_owned()),
                Box::new(patch.pack.to_owned()),
                Box::new(patch.dependency.to_owned()),
                Box::new(patch.src_dir_hash.to_owned()),
            ],
            PatchFilter::ByNameAndPackExact { name, pack }
            | PatchFilter::QueryByNameAndPackSimilar { name, pack } => {
                vec![Box::new(name.to_owned()), Box::new(pack.to_owned())]
            }
            PatchFilter::QueryByDepAndPackExact { dependency, pack } => {
                vec![Box::new(dependency.to_owned()), Box::new(pack.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchFilter::Insert { patch } => Error::Patch(PatchError::NameExists(
                patch.name.to_owned(),
                patch.pack.to_owned(),
            )),
            PatchFilter::ByNameAndPackExact { name, pack }
            | PatchFilter::QueryByNameAndPackSimilar { name, pack } => {
                Error::Patch(PatchError::NotFound(name.to_owned(), pack.to_owned()))
            }
            PatchFilter::QueryByDepAndPackExact { dependency, pack } => Error::Patch(
                PatchError::DepNotFound(dependency.to_owned(), pack.to_owned()),
            ),
        }
    }
}

impl InsertableFilter<Patch> for PatchFilter {
    fn insert(value: Patch) -> Self {
        Self::Insert { patch: value }
    }
}
