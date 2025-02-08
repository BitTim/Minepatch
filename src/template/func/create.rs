/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 01:59
 */
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::{Template, TemplateError};
use rusqlite::Connection;

pub fn create(
    connection: &Connection,
    name: &str,
    loader: Option<String>,
    version: Option<String>,
    download: Option<String>,
) -> Result<()> {
    let exists_query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };
    if TemplateRepo::exists(connection, &exists_query)? {
        return Err(Error::Template(TemplateError::NameTaken(name.to_owned())));
    }

    TemplateRepo::insert(connection, Template::new(name, loader, version, download))?;
    Ok(())
}
