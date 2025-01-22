/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 02:28
 */

use crate::pack::Pack;
use crate::prelude::*;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str) -> Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM pack WHERE name = ?1")?;
    Ok(statement.exists(params![name])?)
}

pub(crate) fn insert(connection: &Connection, value: Pack) -> Result<i64> {
    let mut statement =
        connection.prepare("INSERT INTO pack (name, description, template) VALUES (?1, ?2, ?3)")?;
    Ok(statement.insert(params![value.name, value.description, value.template,])?)
}
