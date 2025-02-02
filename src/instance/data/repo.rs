/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 01:24
 */
use crate::instance::data::queries::InstanceQuery;
use crate::instance::data::Instance;
use crate::instance::InstanceError;
use crate::prelude::*;
use rusqlite::{params, params_from_iter, Connection};
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

pub(crate) fn query_single(
    connection: &Connection,
    query: InstanceQuery,
    params: Vec<String>,
) -> Result<Instance> {
    let mut statement = connection.prepare(&query.value())?;
    let raw_results =
        statement.query_map(params_from_iter(params), |row| Ok(Instance::from_row(row)))?;

    // TODO: Refactor
    let mut results = vec![];
    for result in raw_results {
        let result = result?;
        results.push(result?);
    }

    // TODO: Find better error type
    Ok(results
        .first()
        .ok_or(Error::Instance(InstanceError::NameNotFound("".to_owned())))?
        .clone())
}

pub(crate) fn query_filtered(connection: &Connection, name: Option<&str>) -> Result<Vec<Instance>> {
    let mut statement = connection.prepare(&InstanceQuery::GeneralFilter.value())?;
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

pub(crate) fn query_exact(connection: &Connection, name: &str) -> Result<Instance> {
    let mut statement = connection.prepare_cached(&InstanceQuery::ByName.value())?; // TODO: Put queries into enums
    let raw_results = statement.query_map(params![name], |row| Ok(Instance::from_row(row)))?;

    // TODO: Refactor
    let mut results = vec![];
    for result in raw_results {
        let result = result?;
        results.push(result?);
    }

    Ok(results
        .first()
        .ok_or(Error::Instance(InstanceError::NameNotFound(
            name.to_owned(),
        )))?
        .clone())
}
