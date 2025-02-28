/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::db::Repo;
use crate::patch::{Patch, PatchFilter, PatchRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(conn: &Connection, name: &str, bundle: &str) -> Result<Patch> {
    let query = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchRepo::query_single(conn, &query)
}

pub fn query_multiple(
    conn: &Connection,
    name: Option<&str>,
    bundle: Option<&str>,
) -> Result<HashSet<Patch>> {
    let query = PatchFilter::ByNameAndPackSimilar {
        name: name.unwrap_or_default().to_owned(),
        bundle: bundle.unwrap_or_default().to_owned(),
    };
    PatchRepo::query_multiple(conn, &query)
}

pub fn query_by_dependency_single(
    conn: &Connection,
    dependency: &str,
    bundle: &str,
) -> Result<Patch> {
    let query = PatchFilter::ByDepAndPackExact {
        dependency: dependency.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchRepo::query_single(conn, &query)
}

pub fn query_by_pack_multiple(conn: &Connection, bundle: &str) -> Result<HashSet<Patch>> {
    let query = PatchFilter::ByPackExact {
        bundle: bundle.to_owned(),
    };
    PatchRepo::query_multiple(conn, &query)
}
