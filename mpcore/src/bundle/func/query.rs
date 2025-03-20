/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:26
 */
use crate::bundle::data::{BundleFilter, BundleRepo};
use crate::bundle::Bundle;
use crate::db::Repo;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(conn: &Connection, name: &str) -> Result<Bundle> {
    let filter = BundleFilter::QueryExactName {
        name: name.to_owned(),
    };
    BundleRepo::query_single(conn, &filter)
}

pub fn query_multiple(conn: &Connection, name: Option<&str>) -> Result<HashSet<Bundle>> {
    let filter = BundleFilter::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    BundleRepo::query_multiple(conn, &filter)
}
