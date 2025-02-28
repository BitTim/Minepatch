/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:36
 */
use crate::bundle::Bundle;
use crate::bundle::data::{BundleFilter, BundleRepo};
use crate::db::Repo;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query(conn: &Connection, name: Option<&str>) -> Result<HashSet<Bundle>> {
    let query = BundleFilter::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    BundleRepo::query_multiple(conn, &query)
}
