/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 16:40
 */
use crate::prelude::*;
use crate::template::{data, Template, TemplateError};
use rusqlite::Connection;

pub fn create(
    connection: &Connection,
    name: &str,
    loader: Option<String>,
    version: Option<String>,
    download: Option<String>,
) -> Result<()> {
    if data::exists(connection, name)? {
        return Err(Error::Template(TemplateError::NameTaken(name.to_owned())));
    }

    data::insert(connection, Template::new(name, loader, version, download))?;
    Ok(())
}
