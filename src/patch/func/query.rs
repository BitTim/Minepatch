/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 02:53
 */
use crate::patch::{data, Patch};
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(
    connection: &Connection,
    name: Option<String>,
    pack: Option<String>,
) -> Result<Vec<Patch>> {
    data::query(connection, name, pack)
}
