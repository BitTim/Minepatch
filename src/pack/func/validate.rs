/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:57
 */
use crate::db::Repo;
use crate::pack::data::{PackFilter, PackRepo};
use crate::{patch, template};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> bool {
    let query = PackFilter::QueryExactName {
        name: name.to_owned(),
    };

    let pack = match PackRepo::query_single(connection, &query) {
        Ok(result) => result,
        Err(_) => return false,
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
    let patches = match patch::query_multiple(connection, None, Some(name)) {
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
