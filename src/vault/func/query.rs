/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 00:06
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::data::{ModFilter, VaultRepo};
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_multiple(
    conn: &Connection,
    hash: Option<&str>,
    id: Option<&str>,
    name: Option<&str>,
) -> Result<HashSet<Mod>> {
    let query = ModFilter::QueryHashAndIDAndNameSimilar {
        hash: hash.unwrap_or_default().to_owned(),
        mod_id: id.unwrap_or_default().to_owned(),
        name: name.unwrap_or_default().to_owned(),
    };
    VaultRepo::query_multiple(conn, &query)
}

pub fn query_single(conn: &Connection, hash: &str) -> Result<Mod> {
    let query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };
    VaultRepo::query_single(conn, &query)
}

pub fn query_single_loose_hash(conn: &Connection, hash: &str) -> Result<Mod> {
    let query = ModFilter::QueryHashAndIDAndNameSimilar {
        hash: hash.to_owned(),
        mod_id: "".to_string(),
        name: "".to_string(),
    };
    VaultRepo::query_single(conn, &query)
}
