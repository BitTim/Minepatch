/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:21
 */
use crate::instance::{data, Instance};
use crate::prelude::*;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Instance>> {
    data::query(connection, name)
}
