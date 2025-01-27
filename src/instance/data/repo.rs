/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:10
 */
use crate::instance::data::Instance;
use crate::prelude::*;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub(crate) fn exists(connection: &Connection, name: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM instance WHERE name = ?1")?;
    Ok(statement.exists(params![name])?)
}

pub(crate) fn insert(connection: &Connection, instance: Instance) -> Result<i64> {
    let mut statement = connection
        .prepare("INSERT INTO instance (name, path, pack, patch) VALUES (?1, ?2, ?3, ?4)")?;
    Ok(statement.insert(params![
        instance.name,
        instance.path.display().to_string(),
        instance.pack,
        instance.patch
    ])?)
}

pub(crate) fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Instance>> {
    let mut statement = connection
        .prepare("SELECT name, path, pack, patch FROM instance WHERE name LIKE ?1||'%'")?;
    let raw_results = statement.query_map(params![name.unwrap_or_default()], |row| {
        let path: String = row.get(1)?;
        let path = PathBuf::from(path);

        Ok(Instance {
            name: row.get(0)?,
            path,
            pack: row.get(2)?,
            patch: row.get(3)?,
        })
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}
