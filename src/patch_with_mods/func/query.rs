/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:59
 */
use crate::db::Repo;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(
    connection: &Connection,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<PatchWithMods> {
    let query = PatchModRelFilter::ByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    PatchModRelRepo::query_single(connection, &query)
}

pub fn query_multiple(
    connection: &Connection,
    name: &str,
    pack: &str,
) -> Result<HashSet<PatchWithMods>> {
    let query = PatchModRelFilter::QueryByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };
    PatchModRelRepo::query_multiple(connection, &query)
}
