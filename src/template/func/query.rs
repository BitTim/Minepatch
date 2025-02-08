/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:59
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::Template;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<HashSet<Template>> {
    let query = TemplateFilter::QueryNameSimilar {
        name: name.unwrap_or_default().to_owned(),
    };
    TemplateRepo::query_multiple(connection, &query)
}
