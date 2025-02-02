/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:25
 */
use crate::common::Query;
use crate::instance::data::query::InstanceQuery;
use crate::instance::data::Instance;
use crate::prelude::*;
use rusqlite::{params, params_from_iter, Connection};

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

pub(crate) fn query_single(connection: &Connection, query: InstanceQuery) -> Result<Instance> {
    let mut statement = connection.prepare(&query.value())?;
    let result = statement
        .query_row(params_from_iter(query.params()), |row| {
            Ok(Instance::from_row(row))
        })?
        .map_err(|_| query.error());

    Ok(result?.clone())
}

pub(crate) fn query_multiple(
    connection: &Connection,
    query: InstanceQuery,
) -> Result<Vec<Instance>> {
    let mut statement = connection.prepare(&query.value())?;
    let raw_results = statement.query_map(params_from_iter(query.params()), |row| {
        Ok(Instance::from_row(row))
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result??);
    }

    Ok(results)
}
