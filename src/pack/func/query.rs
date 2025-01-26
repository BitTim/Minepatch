/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 22:19
 */
use crate::pack::{data, Pack};
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<String>) -> Result<Vec<Pack>> {
    data::query(connection, name)
}
