/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       filter.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 13:00
 */
use crate::bundle::{Bundle, BundleError};
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use rusqlite::ToSql;

pub(crate) enum BundleFilter {
    Insert { bundle: Bundle },
    ByExactName { name: String },
    BySimilarName { name: String },
}

impl Filter for BundleFilter {
    fn value(&self) -> String {
        match self {
            BundleFilter::Insert { .. } => "VALUES (?1, ?2, ?3)",
            BundleFilter::ByExactName { .. } => "WHERE name = ?1",
            BundleFilter::BySimilarName { .. } => "WHERE name LIKE ?1||'%'",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            BundleFilter::Insert { bundle } => bundle.to_values(),
            BundleFilter::ByExactName { name } => vec![Box::new(name.to_owned())],
            BundleFilter::BySimilarName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            BundleFilter::Insert { bundle } => {
                Error::Bundle(BundleError::NameTaken(bundle.name.to_owned()))
            }
            BundleFilter::ByExactName { name } | BundleFilter::BySimilarName { name } => {
                Error::Bundle(BundleError::NotFound(name.to_owned()))
            }
        }
    }
}

impl InsertableFilter<Bundle> for BundleFilter {
    fn insert(value: Bundle) -> Self {
        Self::Insert { bundle: value }
    }
}

impl BundleFilter {
    pub(crate) fn build_name_filter(name: Option<&str>, exact: bool) -> Self {
        let name = name.unwrap_or_default().to_owned();

        match exact {
            false => Self::BySimilarName { name },
            true => Self::ByExactName { name },
        }
    }
}
