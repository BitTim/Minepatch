/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 20:55
 */
use crate::patch::data::query;
use crate::patch::Patch;
use crate::patch_with_mods::query_by_patch;
use crate::{pack, vault};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, pack: &str) -> bool {
    let query_result = match query(connection, Some(name.to_owned()), Some(pack.to_owned())) {
        Ok(result) => result,
        Err(_) => return false,
    };

    let patch = match query_result.first() {
        Some(patch) => patch,
        None => return false,
    };

    if !pack::validate(connection, pack)
        || !validate_patch_dependency(connection, patch)
        || !validate_mods(connection, name, pack)
    {
        return false;
    }

    true
}

fn validate_patch_dependency(connection: &Connection, patch: &Patch) -> bool {
    if !patch.dependency.is_empty() {
        return validate(connection, &patch.dependency, &patch.pack);
    }
    true
}

fn validate_mods(connection: &Connection, name: &str, pack: &str) -> bool {
    let mods = match query_by_patch(connection, name, pack) {
        Ok(mods) => mods,
        Err(_) => return false,
    };

    for value in mods {
        if !vault::validate(connection, &value.mod_hash) {
            return false;
        }
    }

    true
}
