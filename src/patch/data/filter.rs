/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::db::{Filter, InsertableFilter};
use crate::error::Error;
use crate::patch::{Patch, PatchError};
use rusqlite::ToSql;

pub(crate) enum PatchFilter {
    Insert { patch: Patch },
    ByNameAndPackExact { name: String, bundle: String },
    ByNameAndPackSimilar { name: String, bundle: String },
    ByDepAndPackExact { dependency: String, bundle: String },
    ByPackExact { bundle: String },
}

impl Filter for PatchFilter {
    fn value(&self) -> String {
        match self {
            PatchFilter::Insert { .. } => "VALUES (?1, ?2, ?3)",
            PatchFilter::ByNameAndPackExact { .. } => "WHERE name = ?1 AND bundle = ?2",
            PatchFilter::ByNameAndPackSimilar { .. } => {
                "WHERE name LIKE ?1||'%' AND bundle LIKE ?2||'%'"
            }
            PatchFilter::ByDepAndPackExact { .. } => "WHERE dependency = ?1 AND bundle = ?2",
            PatchFilter::ByPackExact { .. } => "WHERE bundle = ?1",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            PatchFilter::Insert { patch } => vec![
                Box::new(patch.name.to_owned()),
                Box::new(patch.bundle.to_owned()),
                Box::new(patch.dependency.to_owned()),
            ],
            PatchFilter::ByNameAndPackExact { name, bundle }
            | PatchFilter::ByNameAndPackSimilar { name, bundle } => {
                vec![Box::new(name.to_owned()), Box::new(bundle.to_owned())]
            }
            PatchFilter::ByDepAndPackExact { dependency, bundle } => {
                vec![Box::new(dependency.to_owned()), Box::new(bundle.to_owned())]
            }
            PatchFilter::ByPackExact { bundle } => {
                vec![Box::new(bundle.to_owned())]
            }
        }
    }

    fn error(&self) -> Error {
        match self {
            PatchFilter::Insert { patch } => Error::Patch(PatchError::NameExists {
                name: patch.name.to_owned(),
                bundle: patch.bundle.to_owned(),
            }),
            PatchFilter::ByNameAndPackExact { name, bundle }
            | PatchFilter::ByNameAndPackSimilar { name, bundle } => {
                Error::Patch(PatchError::NotFound {
                    name: name.to_owned(),
                    bundle: bundle.to_owned(),
                })
            }
            PatchFilter::ByDepAndPackExact { dependency, bundle } => {
                Error::Patch(PatchError::DepNotFound {
                    dependency: dependency.to_owned(),
                    bundle: bundle.to_owned(),
                })
            }
            PatchFilter::ByPackExact { bundle } => Error::Patch(PatchError::PackNotFound {
                bundle: bundle.to_owned(),
            }),
        }
    }
}

impl InsertableFilter<Patch> for PatchFilter {
    fn insert(value: Patch) -> Self {
        Self::Insert { patch: value }
    }
}
