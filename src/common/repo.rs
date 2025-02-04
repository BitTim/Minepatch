/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:24
 */
use crate::common::{Query, QueryInsert, QueryModel};
use crate::prelude::*;
use rusqlite::{params_from_iter, Connection};

pub(crate) trait Repo<Q, T>
where
    Q: Query + QueryInsert<T>,
    T: QueryModel,
{
    fn insert(connection: &Connection, value: T) -> Result<i64> {
        let query = Q::insert(value);
        let mut statement = connection.prepare(&query.value())?;
        Ok(statement.insert(params_from_iter(query.params()))?)
    }

    fn exists(connection: &Connection, query: Q) -> Result<bool> {
        let mut statement = connection.prepare(&query.value())?;
        Ok(statement.exists(params_from_iter(query.params()))?)
    }

    fn query_single(connection: &Connection, query: Q) -> Result<T> {
        let mut statement = connection.prepare(&query.value())?;
        let result = statement
            .query_row(params_from_iter(query.params()), |row| Ok(T::from_row(row)))?
            .map_err(|_| query.error());

        Ok(*result?)
    }

    fn query_multiple(connection: &Connection, query: Q) -> Result<Vec<T>> {
        let mut statement = connection.prepare(&query.value())?;
        let raw_results =
            statement.query_map(params_from_iter(query.params()), |row| Ok(T::from_row(row)))?;

        let mut results = vec![];
        for result in raw_results {
            results.push(*result??);
        }

        Ok(results)
    }

    fn remove(connection: &Connection, query: Q) -> Result<usize> {
        Ok(connection.execute(&query.value(), params_from_iter(query.params()))?)
    }
}
