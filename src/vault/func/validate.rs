/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 21:00
 */
use crate::vault::data;
use rusqlite::Connection;
use std::fs;

pub fn validate(connection: &Connection, hash: &str) -> bool {
    let query_result = match data::query(connection, Some(hash.to_owned()), None, None) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let value = match query_result.first() {
        Some(value) => value,
        None => return false,
    };

    fs::exists(&value.path).unwrap_or(false)
}
