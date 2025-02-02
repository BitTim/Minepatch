/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 18:08
 */
use crate::instance::data::InstanceQuery;
use crate::instance::{data, Instance};
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Instance>> {
    let query = InstanceQuery::GeneralFilter {
        name: name.unwrap_or_default().to_owned(),
    };
    data::query_multiple(connection, query)
}
