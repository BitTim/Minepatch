/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:44
 */
use crate::common::db::{Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::{Patch, PatchError};
use rusqlite::ToSql;

pub(crate) enum PatchFilter {
    Insert { patch: Patch },
    ByNameAndPackExact { name: String, pack: String },
    ByNameAndPackSimilar { name: String, pack: String },
    ByDepAndPackExact { dependency: String, pack: String },
    ByPackExact { pack: String },
}

impl Filter for PatchFilter {
    fn value(&self) -> String {
        match self {
            PatchFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            PatchFilter::ByNameAndPackExact { .. } => "WHERE name = ?1 AND pack = ?2",
            PatchFilter::ByNameAndPackSimilar { .. } => {
                "WHERE name LIKE ?1||'%' AND pack LIKE ?2||'%'"
            }
            PatchFilter::ByDepAndPackExact { .. } => "WHERE dependency = ?1 AND pack = ?2",
            PatchFilter::ByPackExact { .. } => "WHERE pack = ?1",
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
            | PatchFilter::ByNameAndPackSimilar { name, pack } => {
                vec![Box::new(name.to_owned()), Box::new(pack.to_owned())]
            }
            PatchFilter::ByDepAndPackExact { dependency, pack } => {
                vec![Box::new(dependency.to_owned()), Box::new(pack.to_owned())]
            }
            PatchFilter::ByPackExact { pack } => {
                vec![Box::new(pack.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchFilter::Insert { patch } => Error::Patch(PatchError::NameExists {
                name: patch.name.to_owned(),
                pack: patch.pack.to_owned(),
            }),
            PatchFilter::ByNameAndPackExact { name, pack }
            | PatchFilter::ByNameAndPackSimilar { name, pack } => {
                Error::Patch(PatchError::NotFound {
                    name: name.to_owned(),
                    pack: pack.to_owned(),
                })
            }
            PatchFilter::ByDepAndPackExact { dependency, pack } => {
                Error::Patch(PatchError::DepNotFound {
                    dependency: dependency.to_owned(),
                    pack: pack.to_owned(),
                })
            }
            PatchFilter::ByPackExact { pack } => Error::Patch(PatchError::PackNotFound {
                pack: pack.to_owned(),
            }),
        }
    }
}

impl InsertableFilter<Patch> for PatchFilter {
    fn insert(value: Patch) -> Self {
        Self::Insert { patch: value }
    }
}
