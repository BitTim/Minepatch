/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:41
 */
use crate::common::Query;
use crate::instance::data::query::InstanceQuery;
use crate::instance::data::Instance;
use crate::prelude::*;
use rusqlite::{params_from_iter, Connection};

pub(crate) fn exists(connection: &Connection, name: &str) -> Result<bool> {
    let query = InstanceQuery::QueryExactName {
        name: name.to_owned(),
    };
    let mut statement = connection.prepare(&query.value())?;
    Ok(statement.exists(params_from_iter(query.params()))?)
}

pub(crate) fn insert(connection: &Connection, instance: Instance) -> Result<i64> {
    let query = InstanceQuery::Insert {
        name: instance.name,
        path: instance.path.display().to_string(),
        pack: instance.pack,
        patch: instance.patch,
    };
    let mut statement = connection.prepare(&query.value())?;
    Ok(statement.insert(params_from_iter(query.params()))?)
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
