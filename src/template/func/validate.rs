/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:59
 */
use crate::db::Repo;
use crate::template::data::{TemplateFilter, TemplateRepo};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> bool {
    let query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };

    TemplateRepo::exists(connection, &query).unwrap_or(false)
}
