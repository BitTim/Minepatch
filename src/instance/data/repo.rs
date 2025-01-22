/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:00
 */
use crate::instance::data::Instance;
use crate::prelude::*;
use rusqlite::{params, Connection};

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
