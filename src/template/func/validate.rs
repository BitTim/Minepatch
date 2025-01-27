/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 09:45
 */
use crate::template::data::query;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    match query(connection, Some(name)) {
        Ok(result) => !result.is_empty(),
        Err(_) => false,
    }
}
