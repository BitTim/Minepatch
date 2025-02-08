/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 21:43
 */
use crate::common::db::SqlAction;
use crate::common::db::{Entity, Filter, InsertableFilter};
use crate::db::build_statement_sql;
use crate::prelude::*;
use rusqlite::{params_from_iter, Connection};
use std::collections::HashSet;

pub(crate) trait Repo<F, T>
where
    F: Filter + InsertableFilter<T>,
    T: Entity,
{
    fn insert(connection: &Connection, value: T) -> Result<i64> {
        let query = F::insert(value);
        let mut statement = connection.prepare(&build_statement_sql(
            SqlAction::Insert,
            &T::table_name(),
            &query.value(),
        ))?;
        Ok(statement.insert(params_from_iter(query.params()))?)
    }

    fn exists(connection: &Connection, filter: &F) -> Result<bool> {
        let mut statement = connection.prepare(&build_statement_sql(
            SqlAction::Select,
            &T::table_name(),
            &filter.value(),
        ))?;
        Ok(statement.exists(params_from_iter(filter.params()))?)
    }

    fn query_single(connection: &Connection, filter: &F) -> Result<T> {
        let mut statement = connection.prepare(&build_statement_sql(
            SqlAction::Select,
            &T::table_name(),
            &filter.value(),
        ))?;

        let result = statement
            .query_row(params_from_iter(filter.params()), |row| {
                Ok(T::from_row(row))
            })
            .map_err(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => filter.error(),
                _ => Error::from(err),
            })?;

        Ok(*result?)
    }

    fn query_multiple(connection: &Connection, filter: &F) -> Result<HashSet<T>> {
        let mut statement = connection.prepare(&build_statement_sql(
            SqlAction::Select,
            &T::table_name(),
            &filter.value(),
        ))?;

        let raw_results = statement
            .query_map(params_from_iter(filter.params()), |row| {
                Ok(T::from_row(row))
            })
            .map_err(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => filter.error(),
                _ => Error::from(err),
            })?;

        let mut results = HashSet::new();
        for result in raw_results {
            results.insert(*result??);
        }

        Ok(results)
    }

    fn remove(connection: &Connection, filter: &F) -> Result<usize> {
        let mut statement = connection.prepare(&build_statement_sql(
            SqlAction::Delete,
            &T::table_name(),
            &filter.value(),
        ))?;
        Ok(statement.execute(params_from_iter(filter.params()))?)
    }
}
