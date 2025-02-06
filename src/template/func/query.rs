/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:59
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::Template;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Template>> {
    let query = TemplateFilter::QueryNameSimilar {
        name: name.unwrap_or_default().to_owned(),
    };
    TemplateRepo::query_multiple(connection, &query)
}
