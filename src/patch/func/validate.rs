/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 03:13
 */
use crate::pack;
use crate::patch::data::query;
use crate::patch_with_mods::query_by_patch;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, pack: &str) -> bool {
    let query_result = query(connection, Some(name.to_owned()), Some(pack.to_owned()));
    if query_result.is_err() {
        return false;
    }

    let query_result = query_result.unwrap();
    let patch = query_result.first();
    if patch.is_none() {
        return false;
    }

    let patch = patch.unwrap();
    let pack_valid = pack::validate(connection, pack);

    let dep_valid = if !patch.dependency.is_empty() {
        pack_valid && validate(connection, &patch.dependency, pack)
    } else {
        true
    };

    let mods = query_by_patch(connection, name, pack);
    if mods.is_err() {
        return false;
    }

    let mods = mods.unwrap();
    for value in mods {
        //TODO: Query and validate mods
    }

    true
}
