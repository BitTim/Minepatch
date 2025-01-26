/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 02:49
 */
use crate::pack::data::query;
use crate::template;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    let query_result = query(connection, name);
    if query_result.is_err() {
        return false;
    }

    let query_result = query_result.unwrap();
    let pack = query_result.first();
    if pack.is_none() {
        return false;
    }

    let pack = pack.unwrap();
    match &pack.template {
        Some(template) => template::validate(connection, template),
        None => true,
    }
}
