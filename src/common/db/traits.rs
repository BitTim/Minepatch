/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       traits.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:18
 */
use crate::error::Error;
use crate::prelude;
use rusqlite::{Row, ToSql};

pub(crate) trait Entity {
    fn table_name() -> String;
    fn from_row(row: &Row) -> prelude::Result<Box<Self>>;
    fn to_params(&self) -> Vec<Box<dyn ToSql>>;
}

pub(crate) trait Filter {
    fn value(&self) -> String;
    fn params(&self) -> Vec<Box<dyn ToSql>>;
    fn error(&self) -> Error;
}

pub(crate) trait InsertableFilter<T> {
    fn insert(value: T) -> Self;
}
