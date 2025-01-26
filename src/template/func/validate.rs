/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 02:47
 */
use crate::template::data::query;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    let query_result = query(connection, name);
    if query_result.is_err() {
        return false;
    }

    !query_result.unwrap().is_empty()
}
