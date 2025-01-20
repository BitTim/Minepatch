/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 17:04
 */

use crate::patch::data::model::Patch;
use rusqlite::{params, Connection};

pub(crate) fn insert(connection: &Connection, value: Patch) -> crate::prelude::Result<i64> {
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
