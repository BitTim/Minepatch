/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:57
 */
use crate::db::Repo;
use crate::pack::data::{PackFilter, PackRepo};
use crate::pack::Pack;
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Pack>> {
    let query = PackFilter::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    PackRepo::query_multiple(connection, &query)
}
