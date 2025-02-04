/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:40
 */
use crate::common::Repo;
use crate::template::data::{TemplateQueries, TemplateRepo};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    let query = TemplateQueries::QueryNameExact {
        name: name.to_owned(),
    };

    TemplateRepo::exists(connection, &query).unwrap_or(false)
}
