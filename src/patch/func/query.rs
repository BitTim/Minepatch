/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:15
 */
use crate::db::Repo;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::Patch;
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
    let query = PatchFilter::QueryByNameAndPackSimilar {
        name: name.unwrap_or_default().to_owned(),
        pack: pack.unwrap_or_default().to_owned(),
    };
    PatchRepo::query_multiple(connection, &query)
}

pub fn query_dependency_single(
    connection: &Connection,
    dependency: &str,
    pack: &str,
) -> Result<Patch> {
    let query = PatchFilter::QueryByDepAndPackExact {
        dependency: dependency.to_owned(),
        pack: pack.to_owned(),
    };
    PatchRepo::query_single(connection, &query)
}
