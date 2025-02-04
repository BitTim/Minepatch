/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:16
 */
use crate::common::Repo;
use crate::patch::data::{PatchQueries, PatchRepo};
use crate::patch::Patch;
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    name: Option<&str>,
    pack: Option<&str>,
) -> Result<Vec<Patch>> {
    let query = PatchQueries::QueryNameAndPackSimilar {
        name: name.unwrap_or_default().to_owned(),
        pack: pack.unwrap_or_default().to_owned(),
    };
    PatchRepo::query_multiple(connection, query)
}
