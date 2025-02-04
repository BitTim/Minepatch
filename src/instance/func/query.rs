/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:16
 */
use crate::common::Repo;
use crate::instance::data::{InstanceQuery, InstanceRepo};
use crate::instance::Instance;
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Instance>> {
    let query = InstanceQuery::QuerySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    InstanceRepo::query_multiple(connection, &query)
}
