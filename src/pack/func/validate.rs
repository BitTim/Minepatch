/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:51
 */
use crate::pack::data::query;
use crate::{patch, template};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> bool {
    let query_result = match query(connection, Some(name)) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let pack = match query_result.first() {
        Some(pack) => pack,
        None => return false,
    };

    if exist_only {
        return true;
    }

    if !validate_template(connection, &pack.template) || !validate_patches(connection, &pack.name) {
        return false;
    }

    true
}

fn validate_template(connection: &Connection, template: &Option<String>) -> bool {
    match template {
        Some(template) => template::validate(connection, template),
        None => true,
    }
}

fn validate_patches(connection: &Connection, name: &str) -> bool {
    let patches = match patch::query(connection, None, Some(name)) {
        Ok(patches) => patches,
        Err(_) => return false,
    };

    for patch in patches {
        if !patch::validate(connection, &patch.name, &patch.pack, false) {
            return false;
        }
    }

    true
}
