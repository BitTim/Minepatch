/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:24
 */
use crate::prelude::Error;
use rusqlite::ToSql;

pub(crate) trait Query {
    fn value(&self) -> String;
    fn params(&self) -> Vec<&(dyn ToSql + '_)>;
    fn error(&self) -> Error;
}
