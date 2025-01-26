/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 22:18
 */
use crate::pack::data::query;
use crate::template;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    let query_result = match query(connection, Some(name.to_owned())) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let pack = match query_result.first() {
        Some(pack) => pack,
        None => return false,
    };

    validate_template(connection, &pack.template)
}

fn validate_template(connection: &Connection, template: &Option<String>) -> bool {
    match template {
        Some(template) => template::validate(connection, template),
        None => true,
    }
}
