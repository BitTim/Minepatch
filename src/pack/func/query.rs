/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:19
 */
use crate::db::Repo;
use crate::pack::data::{PackFilter, PackRepo};
use crate::pack::Pack;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<HashSet<Pack>> {
    let query = PackFilter::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    PackRepo::query_multiple(connection, &query)
}
