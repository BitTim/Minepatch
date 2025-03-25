/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 17:55
 */
use crate::bundle::Bundle;
use crate::bundle::data::filter::BundleFilter;
use crate::db::Repo;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub struct BundleRepo {}

impl Repo<BundleFilter, Bundle> for BundleRepo {}

impl BundleRepo {
    pub fn exists(conn: &Connection, name: &str) -> Result<bool> {
        BundleRepo::exists_by_filter(conn, &BundleFilter::build_name_filter(Some(name), true))
    }

    pub fn by_name(conn: &Connection, name: &str) -> Result<Bundle> {
        BundleRepo::by_filter(conn, &BundleFilter::build_name_filter(Some(name), true))
    }

    pub fn by_name_many(
        conn: &Connection,
        name: Option<&str>,
        exact: bool,
    ) -> Result<HashSet<Bundle>> {
        BundleRepo::by_filter_many(conn, &BundleFilter::build_name_filter(name, exact))
    }
}
