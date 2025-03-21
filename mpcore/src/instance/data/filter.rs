/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 12:59
 */
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::instance::{Instance, InstanceError};
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) enum InstanceFilter {
    Insert { instance: Instance },
    BySimilarName { name: String },
    ByExactName { name: String },
    BySimilarPatch { patch: String },
    ByExactPatch { patch: String },
}

impl Filter for InstanceFilter {
    fn value(&self) -> String {
        match self {
            InstanceFilter::Insert { .. } => "VALUES (?1, ?2, ?3, ?4)",
            InstanceFilter::BySimilarName { .. } => "WHERE name LIKE ?1||'%'",
            InstanceFilter::ByExactName { .. } => "WHERE name = ?1",
            InstanceFilter::BySimilarPatch { .. } => "WHERE patch LIKE ?1||'%'",
            InstanceFilter::ByExactPatch { .. } => "WHERE patch = ?1",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            InstanceFilter::Insert { instance } => instance.to_values(),
            InstanceFilter::BySimilarName { name } => vec![Box::new(name.to_owned())],
            InstanceFilter::ByExactName { name } => vec![Box::new(name.to_owned())],
            InstanceFilter::BySimilarPatch { patch } => vec![Box::new(patch.to_owned())],
            InstanceFilter::ByExactPatch { patch } => vec![Box::new(patch.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            InstanceFilter::Insert { instance } => Error::Instance(InstanceError::NameTaken {
                name: instance.name.to_owned(),
            }),
            InstanceFilter::BySimilarName { name } | InstanceFilter::ByExactName { name } => {
                Error::Instance(InstanceError::NameNotFound {
                    name: name.to_owned(),
                })
            }
            InstanceFilter::BySimilarPatch { patch } | InstanceFilter::ByExactPatch { patch } => {
                Error::Instance(InstanceError::PatchNotFound {
                    patch: patch.to_owned(),
                })
            }
        }
    }
}

impl InsertableFilter<Instance> for InstanceFilter {
    fn insert(value: Instance) -> Self {
        Self::Insert { instance: value }
    }
}

impl InstanceFilter {
    pub(crate) fn build_name_filter(name: Option<&str>, exact: bool) -> InstanceFilter {
        let name = name.unwrap_or_default().to_owned();

        match exact {
            false => InstanceFilter::BySimilarName { name },
            true => InstanceFilter::ByExactName { name },
        }
    }

    pub(crate) fn build_patch_filter(patch: Option<&str>, exact: bool) -> InstanceFilter {
        let patch = patch.unwrap_or_default().to_owned();

        match exact {
            false => InstanceFilter::BySimilarPatch { patch },
            true => InstanceFilter::ByExactPatch { patch },
        }
    }
}
