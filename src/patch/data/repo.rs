/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.01.25, 17:06
 */

use crate::patch::data::model::Patch;
use crate::prelude::*;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str, pack: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM patch WHERE name = ?1 AND pack = ?2")?;
    Ok(statement.exists(params![name, pack])?)
}

pub(crate) fn insert(connection: &Connection, patch: Patch) -> Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO patch (name, dependency, state_hash, pack) VALUES (?1, ?2, ?3, ?4)",
    )?;

    Ok(statement.insert(params![
        patch.name,
        patch.dependency,
        patch.state_hash,
        patch.pack,
    ])?)
}

pub(crate) fn query(connection: &Connection, name: &str, pack: &str) -> Result<Vec<Patch>> {
    let mut statement = connection.prepare(
        "SELECT name, dependency, state_hash, pack FROM patch WHERE name = ?1 AND pack = ?2",
    )?;
    let raw_results = statement.query_map(params![name, pack], |row| {
        Ok(Patch {
            name: name.to_owned(),
            dependency: row.get(1)?,
            state_hash: row.get(2)?,
            pack: pack.to_owned(),
        })
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}
