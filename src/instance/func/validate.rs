/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::db::Repo;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::{pack, patch};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> bool {
    let query = InstanceFilter::QueryExactName {
        name: name.to_owned(),
    };
    let instance = match InstanceRepo::query_single(connection, &query) {
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
