/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 22:17
 */
use crate::db::Repo;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::Instance;
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(connection: &Connection, name: &str) -> Result<Instance> {
    let query = InstanceFilter::ByExactName {
        name: name.to_owned(),
    };
    InstanceRepo::query_single(connection, &query)
}

pub fn query_multiple(connection: &Connection, name: Option<&str>) -> Result<HashSet<Instance>> {
    let query = InstanceFilter::BySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    InstanceRepo::query_multiple(connection, &query)
}
