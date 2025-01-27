/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:04
 */
use crate::prelude::*;
use crate::template::data::model::Template;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM template WHERE name = ?1")?;
    Ok(statement.exists(params![name])?)
}

pub(crate) fn insert(connection: &Connection, template: Template) -> Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO template (name, loader, version, download) VALUES (?1, ?2, ?3, ?4)",
    )?;

    Ok(statement.insert(params![
        template.name,
        template.loader,
        template.version,
        template.download,
    ])?)
}

pub(crate) fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Template>> {
    let mut statement = connection
        .prepare("SELECT name, loader, version, download FROM template WHERE name LIKE ?1||'%'")?;
    let raw_results = statement.query_map(params![name.unwrap_or_default()], |row| {
        Ok(Template {
            name: row.get(0)?,
            loader: row.get(1)?,
            version: row.get(2)?,
            download: row.get(3)?,
        })
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}
