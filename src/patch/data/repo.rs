/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:16
 */

use crate::patch::data::model::Patch;
use crate::prelude::*;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str, pack: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM patch WHERE name = ?1 AND pack = ?2")?;
    Ok(statement.exists(params![name, pack])?)
}

pub(crate) fn insert(connection: &Connection, value: Patch) -> Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO patch (name, dependency, state_hash, pack) VALUES (?1, ?2, ?3, ?4)",
    )?;

    Ok(statement.insert(params![
        value.name,
        value.dependency,
        value.state_hash,
        value.pack,
    ])?)
}
