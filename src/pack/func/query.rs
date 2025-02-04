/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:13
 */
use crate::common::Repo;
use crate::pack::data::{PackQueries, PackRepo};
use crate::pack::Pack;
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Pack>> {
    let query = PackQueries::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    PackRepo::query_multiple(connection, &query)
}
