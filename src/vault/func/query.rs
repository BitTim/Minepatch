/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:15
 */
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    hash: Option<&str>,
    id: Option<&str>,
    name: Option<&str>,
) -> Result<Vec<Mod>> {
    data::query(connection, hash, id, name)
}
