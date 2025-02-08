/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   07.02.25, 17:15
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::TemplateError;
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str) -> Result<()> {
    let query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };

    if !TemplateRepo::exists(connection, &query)? {
        return Err(Error::Template(TemplateError::NotFound(name.to_owned())));
    }

    Ok(())
}
