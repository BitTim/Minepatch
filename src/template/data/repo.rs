/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 18:00
 */
use crate::template::data::model::Template;
use rusqlite::{params, Connection};

pub(crate) fn exists(connection: &Connection, name: &str) -> crate::prelude::Result<bool> {
    let mut statement = connection.prepare("SELECT * FROM template WHERE name = ?1")?;
    Ok(statement.exists(params![name])?)
}

pub(crate) fn insert(connection: &Connection, template: Template) -> crate::prelude::Result<i64> {
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
