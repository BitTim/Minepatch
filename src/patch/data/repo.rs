/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 03:09
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
        "INSERT INTO patch (name, dependency, src_dir_hash, pack) VALUES (?1, ?2, ?3, ?4)",
    )?;

    Ok(statement.insert(params![
        patch.name,
        patch.dependency,
        patch.src_dir_hash,
        patch.pack,
    ])?)
}

pub(crate) fn query(
    connection: &Connection,
    name: Option<String>,
    pack: Option<String>,
) -> Result<Vec<Patch>> {
    let mut statement = connection.prepare(
        "SELECT name, dependency, src_dir_hash, pack FROM patch WHERE name LIKE ?1||'%' AND pack LIKE ?2||'%'",
    )?;
    let raw_results = statement.query_map(
        params![name.unwrap_or_default(), pack.unwrap_or_default()],
        |row| {
            Ok(Patch {
                name: row.get(0)?,
                dependency: row.get(1)?,
                src_dir_hash: row.get(2)?,
                pack: row.get(3)?,
            })
        },
    )?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}
