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
use crate::instance::data;
use crate::{pack, patch};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> bool {
    let query_result = match data::query(connection, Some(name)) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let instance = match query_result.first() {
        Some(instance) => instance,
        None => return false,
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
