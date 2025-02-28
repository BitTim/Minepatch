/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:43
 */
use crate::db::Repo;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(
    conn: &Connection,
    name: &str,
    bundle: &str,
    mod_hash: &str,
) -> Result<PatchWithMods> {
    let query = PatchModRelFilter::ByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    PatchModRelRepo::query_single(conn, &query)
}

pub fn query_multiple(
    conn: &Connection,
    name: &str,
    bundle: &str,
) -> Result<HashSet<PatchWithMods>> {
    let query = PatchModRelFilter::ByPatchAndPackExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchModRelRepo::query_multiple(conn, &query)
}
