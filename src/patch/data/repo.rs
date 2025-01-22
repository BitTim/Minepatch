/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 03:36
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

pub(crate) fn query_relation(
    connection: &Connection,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<Option<bool>> {
    let mut statement = connection
        .prepare("SELECT * FROM patch_with_mods WHERE patch = ?1 AND pack = ?2 AND mod = ?3")?;

    let raw_results = statement.query_and_then(params![name, pack, mod_hash], |row| {
        Ok::<bool, Error>(row.get(3)?)
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results.first().copied())
}

pub(crate) fn insert_relation(
    connection: &Connection,
    name: &str,
    pack: &str,
    mod_hash: &str,
    removed: bool,
) -> Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO patch_with_mods (patch, pack, mod, removed) VALUES (?1, ?2, ?3, ?4)",
    )?;
    Ok(statement.insert(params![name, pack, mod_hash, removed])?)
}
