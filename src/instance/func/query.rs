/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 10:41
 */
use crate::db::Repo;
use crate::instance::Instance;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(conn: &Connection, name: &str) -> Result<Instance> {
    let query = InstanceFilter::ByExactName {
        name: name.to_owned(),
    };
    InstanceRepo::query_single(conn, &query)
}

pub fn query_multiple(conn: &Connection, name: Option<&str>) -> Result<HashSet<Instance>> {
    let query = InstanceFilter::BySimilarName {
        name: name.unwrap_or_default().to_owned(),
    };
    InstanceRepo::query_multiple(conn, &query)
}

pub(crate) fn query_single_by_patch(conn: &Connection, patch: &str) -> Result<Instance> {
    let query = InstanceFilter::ByExactPatch {
        patch: patch.to_owned(),
    };
    InstanceRepo::query_single(conn, &query)
}
