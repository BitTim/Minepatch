/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 11:41
 */
use crate::vault::data;
use rusqlite::Connection;
use std::fs;

pub fn validate(connection: &Connection, hash: &str) -> bool {
    let query_result = match data::query_filtered(connection, Some(hash), None, None) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let value = match query_result.first() {
        Some(value) => value,
        None => return false,
    };

    fs::exists(&value.path).unwrap_or(false)
}
