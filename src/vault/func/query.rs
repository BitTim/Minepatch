/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:00
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::{Mod, ModFilter, VaultRepo};
use rusqlite::Connection;

pub fn query_multiple(
    connection: &Connection,
    hash: Option<&str>,
    id: Option<&str>,
    name: Option<&str>,
) -> Result<Vec<Mod>> {
    let query = ModFilter::QueryHashAndIDAndNameSimilar {
        hash: hash.unwrap_or_default().to_owned(),
        mod_id: id.unwrap_or_default().to_owned(),
        name: name.unwrap_or_default().to_owned(),
    };
    VaultRepo::query_multiple(connection, &query)
}

pub fn query_single(connection: &Connection, hash: &str) -> Result<Mod> {
    let query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };
    VaultRepo::query_single(connection, &query)
}
