/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Repo;
use crate::legacy::template::data::Template;
use crate::legacy::template::data::{TemplateFilter, TemplateRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn query_single(conn: &Connection, name: &str) -> Result<Template> {
    let query = TemplateFilter::QueryNameSimilar {
        name: name.to_owned(),
    };
    TemplateRepo::query_single(conn, &query)
}

pub fn query_multiple(conn: &Connection, name: Option<&str>) -> Result<HashSet<Template>> {
    let query = TemplateFilter::QueryNameSimilar {
        name: name.unwrap_or_default().to_owned(),
    };
    TemplateRepo::query_multiple(conn, &query)
}
