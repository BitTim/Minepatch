/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:13
 */
use crate::common::Repo;
use crate::patch::data::{PatchQueries, PatchRepo};
use crate::patch::Patch;
use crate::patch_with_mods::{PatchModRelQueries, PatchModRelRepo};
use crate::{pack, vault};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, pack: &str, exist_only: bool) -> bool {
    let query = PatchQueries::QueryNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = match PatchRepo::query_single(connection, &query) {
        Ok(result) => result,
        Err(_) => return false,
    };

    if exist_only {
        return true;
    }

    if !pack::validate(connection, pack, true)
        || !validate_patch_dependency(connection, &patch)
        || !validate_mods(connection, name, pack)
    {
        return false;
    }

    true
}

fn validate_patch_dependency(connection: &Connection, patch: &Patch) -> bool {
    if !patch.dependency.is_empty() {
        return validate(connection, &patch.dependency, &patch.pack, false);
    }
    true
}

fn validate_mods(connection: &Connection, name: &str, pack: &str) -> bool {
    let query = PatchModRelQueries::QueryByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };
    let mods = match PatchModRelRepo::query_multiple(connection, &query) {
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
