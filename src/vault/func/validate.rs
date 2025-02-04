/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:29
 */
use crate::common::Repo;
use crate::vault::data::{VaultQueries, VaultRepo};
use rusqlite::Connection;
use std::fs;

pub fn validate(connection: &Connection, hash: &str) -> bool {
    let query = VaultQueries::QueryHashExact {
        hash: hash.to_owned(),
    };
    let value = match VaultRepo::query_single(connection, &query) {
        Ok(result) => result,
        Err(_) => return false,
    };

    fs::exists(&value.path).unwrap_or(false)
}
