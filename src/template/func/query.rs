/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       query.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:37
 */
use crate::common::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateQueries, TemplateRepo};
use crate::template::Template;
use rusqlite::Connection;

pub fn query(connection: &Connection, name: Option<&str>) -> Result<Vec<Template>> {
    let query = TemplateQueries::QueryNameSimilar {
        name: name.unwrap_or_default().to_owned(),
    };
    TemplateRepo::query_multiple(connection, &query)
}
