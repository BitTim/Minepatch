/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.01.25, 16:48
 */
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    hash: Option<String>,
    id: Option<String>,
    name: Option<String>,
) -> Result<Vec<Mod>> {
    data::query(connection, hash, id, name)
}
