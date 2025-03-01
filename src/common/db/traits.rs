/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       traits.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:16
 */
use crate::error::Error;
use crate::prelude;
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub(crate) trait Entity: Eq + PartialEq + Hash {
    fn table_name() -> String;
    fn from_row(row: &Row) -> prelude::Result<Box<Self>>;
    fn to_params(&self) -> Vec<Box<dyn ToSql>>;
}

pub(crate) trait Portable<'a>: Eq + PartialEq + Hash + Serialize + Deserialize<'a> {
    fn file_extension() -> String;
    fn object_name(&self) -> String;
}

pub(crate) trait Filter {
    fn value(&self) -> String;
    fn params(&self) -> Vec<Box<dyn ToSql>>;
    fn error(&self) -> Error;
}

pub(crate) trait InsertableFilter<T> {
    fn insert(value: T) -> Self;
}
