/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Repo;
use crate::legacy::patch::{Patch, PatchFilter, PatchRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(conn: &Connection, name: &str, bundle: &str) -> Result<Patch> {
    let query = PatchFilter::ByNameAndBundleExact {
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
    let query = PatchFilter::ByNameAndBundleSimilar {
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
    let query = PatchFilter::ByDepAndBundleExact {
        dependency: dependency.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchRepo::query_single(conn, &query)
}

pub fn query_by_pack_multiple(conn: &Connection, bundle: &str) -> Result<HashSet<Patch>> {
    let query = PatchFilter::ByBundleExact {
        bundle: bundle.to_owned(),
    };
    PatchRepo::query_multiple(conn, &query)
}
