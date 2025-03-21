/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 06:12
 */
use crate::db::Repo;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchModRelation};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(
    conn: &Connection,
    name: &str,
    bundle: &str,
    mod_hash: &str,
) -> Result<PatchModRelation> {
    let query = PatchModRelFilter::ByPatchAndBundleAndModHashExact {
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
) -> Result<HashSet<PatchModRelation>> {
    let query = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchModRelRepo::query_multiple(conn, &query)
}

pub fn query_multiple_by_bundle(
    conn: &Connection,
    bundle: &str,
) -> Result<HashSet<PatchModRelation>> {
    let query = PatchModRelFilter::ByBundleExact {
        bundle: bundle.to_owned(),
    };
    PatchModRelRepo::query_multiple(conn, &query)
}
