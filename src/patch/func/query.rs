/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:43
 */
use crate::db::Repo;
use crate::patch::{Patch, PatchFilter, PatchRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(connection: &Connection, name: &str, pack: &str) -> Result<Patch> {
    let query = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    PatchRepo::query_single(connection, &query)
}

pub fn query_multiple(
    connection: &Connection,
    name: Option<&str>,
    pack: Option<&str>,
) -> Result<HashSet<Patch>> {
    let query = PatchFilter::ByNameAndPackSimilar {
        name: name.unwrap_or_default().to_owned(),
        pack: pack.unwrap_or_default().to_owned(),
    };
    PatchRepo::query_multiple(connection, &query)
}

pub fn query_by_dependency_single(
    connection: &Connection,
    dependency: &str,
    pack: &str,
) -> Result<Patch> {
    let query = PatchFilter::ByDepAndPackExact {
        dependency: dependency.to_owned(),
        pack: pack.to_owned(),
    };
    PatchRepo::query_single(connection, &query)
}

pub fn query_by_pack_multiple(connection: &Connection, pack: &str) -> Result<HashSet<Patch>> {
    let query = PatchFilter::ByPackExact {
        pack: pack.to_owned(),
    };
    PatchRepo::query_multiple(connection, &query)
}
