/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 15:14
 */
use crate::pack::Pack;
use rusqlite::{params, Connection};

pub(crate) fn insert(connection: &Connection, value: Pack) -> crate::prelude::Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO pack (name, description, base, latest_patch) VALUES (?1, ?2, ?3, ?4)",
    )?;
    Ok(statement.insert(params![
        value.name,
        value.description,
        value.base,
        value.latest_patch,
    ])?)
}
