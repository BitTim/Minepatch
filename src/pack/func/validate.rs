/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   07.02.25, 17:17
 */
use crate::db::Repo;
use crate::pack::data::{PackFilter, PackRepo};
use crate::prelude::*;
use crate::{patch, template};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, exist_only: bool) -> Result<()> {
    let query = PackFilter::QueryExactName {
        name: name.to_owned(),
    };

    let pack = PackRepo::query_single(connection, &query)?;

    if exist_only {
        return Ok(());
    }

    validate_template(connection, &pack.template)?;
    validate_patches(connection, &pack.name)?;

    Ok(())
}

fn validate_template(connection: &Connection, template: &Option<String>) -> Result<()> {
    match template {
        Some(template) => template::validate(connection, template),
        None => Ok(()),
    }
}

fn validate_patches(connection: &Connection, name: &str) -> Result<()> {
    let patches = patch::query_multiple(connection, None, Some(name))?;

    for patch in patches {
        patch::validate(connection, &patch.name, &patch.pack, false)?
    }

    Ok(())
}
