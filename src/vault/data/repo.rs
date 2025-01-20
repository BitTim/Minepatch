/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 16:38
 */
use crate::meta::data::Meta;
use crate::prelude::*;
use crate::vault::data::model::Mod;
use ron::{from_str, to_string};
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub(crate) fn exists(connection: &Connection, hash: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM mod WHERE hash = ?1")?;
    Ok(statement.exists(params![hash])?)
}

pub(crate) fn insert(connection: &Connection, value: Mod) -> Result<i64> {
    let mut statement = connection.prepare("INSERT INTO mod (hash, path, modid, name, version, description, authors, loader, loader_version, mc_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?;
    let authors = value
        .meta
        .authors
        .map(|authors| to_string(&authors))
        .transpose()?;

    Ok(statement.insert(params![
        value.hash,
        value.path.display().to_string(),
        value.meta.id,
        value.meta.name,
        value.meta.version,
        value.meta.description,
        authors,
        value.meta.loader,
        value.meta.loader_version,
        value.meta.minecraft_version,
    ])?)
}

pub(crate) fn query(connection: &Connection, hash: &str, id: &str, name: &str) -> Result<Vec<Mod>> {
    let mut statement = connection
        .prepare("SELECT * FROM mod WHERE hash LIKE ?1||'%' AND modid LIKE '%'||?2||'%' AND name LIKE '%'||?3||'%'")?;
    let raw_results = statement.query_and_then(params![hash, id, name], |row| {
        let path: String = row.get(1)?;
        let authors: Option<String> = row.get(6)?;

        Ok::<Mod, Error>(Mod {
            hash: row.get(0)?,
            path: PathBuf::from(&path),
            meta: Meta {
                id: row.get(2)?,
                name: row.get(3)?,
                version: row.get(4)?,
                description: row.get(5)?,
                authors: authors.map(|authors| from_str(&authors)).transpose()?,
                loader: row.get(7)?,
                loader_version: row.get(8)?,
                minecraft_version: row.get(9)?,
            },
        })
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}

pub(crate) fn remove(connection: &Connection, hash: &str) -> Result<usize> {
    Ok(connection.execute("DELETE FROM mod WHERE hash = ?1", params![hash])?)
}
