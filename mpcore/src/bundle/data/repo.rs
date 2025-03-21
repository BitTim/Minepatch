/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 07:29
 */
use crate::bundle::data::filter::BundleFilter;
use crate::bundle::Bundle;
use crate::db::Repo;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub(crate) struct BundleRepo {}

impl Repo<BundleFilter, Bundle> for BundleRepo {}

impl BundleRepo {
    fn build_filter(name: Option<&str>, similar: bool) -> BundleFilter {
        let name = name.unwrap_or_default().to_owned();

        match similar {
            true => BundleFilter::QuerySimilarName { name },
            false => BundleFilter::QueryExactName { name },
        }
    }

    pub fn exists(conn: &Connection, name: &str) -> Result<bool> {
        BundleRepo::exists_by_filter(conn, &Self::build_filter(Some(name), false))
    }

    pub fn by_name(conn: &Connection, name: &str) -> Result<Bundle> {
        BundleRepo::by_filter(conn, &Self::build_filter(Some(name), false))
    }

    pub fn by_name_many(conn: &Connection, name: Option<&str>) -> Result<HashSet<Bundle>> {
        BundleRepo::by_filter_many(conn, &Self::build_filter(name, true))
    }
}
