/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:34
 */
use crate::common::event;
use crate::db::Repo;
use crate::pack::data::{PackFilter, PackRepo};
use crate::pack::PackProcess;
use crate::prelude::*;
use crate::{patch, template};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    exist_only: bool,
) -> Result<()> {
    event::init_progress(tx, Process::Pack(PackProcess::Validate), None)?;
    let query = PackFilter::QueryExactName {
        name: name.to_owned(),
    };

    let pack = PackRepo::query_single(connection, &query)?;

    if exist_only {
        event::end_progress(tx, Process::Pack(PackProcess::Validate), None)?;
        return Ok(());
    }

    validate_template(connection, tx, &pack.template)?;
    validate_patches(connection, tx, &pack.name)?;

    event::end_progress(tx, Process::Pack(PackProcess::Validate), None)?;
    Ok(())
}

fn validate_template(
    connection: &Connection,
    tx: &Sender<Event>,
    template: &Option<String>,
) -> Result<()> {
    match template {
        Some(template) => template::validate(connection, tx, template),
        None => Ok(()),
    }
}

fn validate_patches(connection: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    let patches = patch::query_multiple(connection, None, Some(name))?;

    for patch in patches {
        patch::validate(connection, tx, &patch.name, &patch.pack, false)?
    }

    Ok(())
}
