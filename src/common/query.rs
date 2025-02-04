/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 18:22
 */
use crate::prelude::*;
use rusqlite::{Row, ToSql};

pub(crate) trait QueryModel {
    fn from_row(row: &Row) -> Result<Box<Self>>;
    fn to_params(&self) -> Vec<Box<dyn ToSql>>;
}

pub(crate) trait Query {
    fn value(&self) -> String;
    fn params(&self) -> Vec<Box<dyn ToSql>>;
    fn error(&self) -> Error;
}

pub(crate) trait QueryInsert<T> {
    fn insert(value: T) -> Self;
}
