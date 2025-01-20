/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 13:45
 */
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use rusqlite::Connection;

pub fn query(connection: &Connection, hash: &str, id: &str, name: &str) -> Result<Vec<Mod>> {
    data::query(connection, hash, id, name)
}
