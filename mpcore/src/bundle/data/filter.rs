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
use crate::bundle::{Bundle, BundleError};
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::error::Error;
use rusqlite::ToSql;

pub(crate) enum BundleFilter {
    Insert { bundle: Bundle },
    QueryExactName { name: String },
    QuerySimilarName { name: String },
}

impl Filter for BundleFilter {
    fn value(&self) -> String {
        match self {
            BundleFilter::Insert { .. } => "VALUES (?1, ?2, ?3)",
            BundleFilter::QueryExactName { .. } => "WHERE name = ?1",
            BundleFilter::QuerySimilarName { .. } => "WHERE name LIKE ?1||'%'",
        }
        .to_owned()
    }

    fn params(&self) -> Vec<Box<dyn ToSql>> {
        match self {
            BundleFilter::Insert { bundle } => bundle.to_params(),
            BundleFilter::QueryExactName { name } => vec![Box::new(name.to_owned())],
            BundleFilter::QuerySimilarName { name } => vec![Box::new(name.to_owned())],
        }
    }

    fn error(&self) -> Error {
        match self {
            BundleFilter::Insert { bundle } => {
                Error::Bundle(BundleError::NameTaken(bundle.name.to_owned()))
            }
            BundleFilter::QueryExactName { name } | BundleFilter::QuerySimilarName { name } => {
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
