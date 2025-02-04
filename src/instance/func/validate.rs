/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:04
 */
use crate::common::Repo;
use crate::instance::data::{InstanceQuery, InstanceRepo};
use crate::{pack, patch};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> bool {
    let query = InstanceQuery::QueryExactName {
        name: name.to_owned(),
    };
    let instance = match InstanceRepo::query_single(connection, query) {
        Ok(result) => result,
        Err(_) => return false,
    };

    if exist_only {
        return true;
    }

    if !pack::validate(connection, &instance.pack, false)
        || !patch::validate(connection, &instance.patch, &instance.pack, false)
    {
        return false;
    }

    true
}
