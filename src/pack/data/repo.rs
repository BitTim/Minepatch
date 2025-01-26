/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 22:17
 */

use crate::pack::Pack;
use crate::prelude::*;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM pack WHERE name = ?1")?;
    Ok(statement.exists(params![name])?)
}

pub(crate) fn insert(connection: &Connection, pack: Pack) -> Result<i64> {
    let mut statement =
        connection.prepare("INSERT INTO pack (name, description, template) VALUES (?1, ?2, ?3)")?;
    Ok(statement.insert(params![pack.name, pack.description, pack.template,])?)
}

pub(crate) fn query(connection: &Connection, name: Option<String>) -> Result<Vec<Pack>> {
    let mut statement = connection
        .prepare("SELECT name, description, template FROM pack WHERE name LIKE ?1||'%'")?;
    let raw_results = statement.query_map(params![name.unwrap_or_default()], |row| {
        Ok(Pack {
            name: row.get(0)?,
            description: row.get(1)?,
            template: row.get(2)?,
        })
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}
