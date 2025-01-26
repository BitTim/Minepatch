/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 17:52
 */
use crate::vault::data;
use rusqlite::Connection;
use std::fs;

pub fn validate(connection: &Connection, hash: &str) -> bool {
    let query_result = data::query(connection, Some(hash.to_owned()), None, None);
    if query_result.is_err() {
        return false;
    }
    let query_result = query_result.unwrap();

    let value = query_result.first();
    if value.is_none() {
        return false;
    }
    let value = value.unwrap();

    let path_valid = fs::exists(&value.path);
    if path_valid.is_err() {
        return false;
    }
    path_valid.unwrap()
}
