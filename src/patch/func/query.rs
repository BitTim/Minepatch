/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:13
 */
use crate::patch::{data, Patch};
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    name: Option<&str>,
    pack: Option<&str>,
) -> Result<Vec<Patch>> {
    data::query(connection, name, pack)
}
