/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:24
 */
use crate::common::Repo;
use crate::prelude::*;
use crate::vault::data::{Mod, VaultQueries, VaultRepo};
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    hash: Option<&str>,
    id: Option<&str>,
    name: Option<&str>,
) -> Result<Vec<Mod>> {
    let query = VaultQueries::QueryHashAndIDAndNameSimilar {
        hash: hash.unwrap_or_default().to_owned(),
        mod_id: id.unwrap_or_default().to_owned(),
        name: name.unwrap_or_default().to_owned(),
    };
    VaultRepo::query_multiple(connection, &query)
}
