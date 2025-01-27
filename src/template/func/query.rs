/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 09:43
 */
use crate::prelude::*;
use crate::template::{data, Template};
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Template>> {
    data::query(connection, name)
}
