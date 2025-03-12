/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.03.25, 06:45
 */
use crate::bundle::data::{BundleFilter, BundleRepo};
use crate::bundle::{BundleMessage, BundleProcess};
use crate::common::event;
use crate::db::Repo;
use crate::prelude::*;
use crate::{patch, template};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, name: &str, exist_only: bool) -> Result<()> {
    event::init_progress(tx, Process::Bundle(BundleProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Bundle(BundleProcess::Validate),
        Message::Bundle(BundleMessage::ValidateStatus {
            name: name.to_owned(),
        }),
        1,
    )?;
    let query = BundleFilter::QueryExactName {
        name: name.to_owned(),
    };

    let bundle = BundleRepo::query_single(conn, &query)?;

    if exist_only {
        event::end_progress(tx, Process::Bundle(BundleProcess::Validate), None)?;
        return Ok(());
    }

    validate_template(conn, tx, &bundle.template)?;
    validate_patches(conn, tx, &bundle.name)?;

    event::end_progress(tx, Process::Bundle(BundleProcess::Validate), None)?;
    Ok(())
}

fn validate_template(
    conn: &Connection,
    tx: &Sender<Event>,
    template: &Option<String>,
) -> Result<()> {
    match template {
        Some(template) => template::validate(conn, tx, template),
        None => Ok(()),
    }
}

fn validate_patches(conn: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    let patches = patch::query_multiple(conn, None, Some(name))?;

    for patch in patches {
        patch::validate(conn, tx, &patch.name, &patch.bundle, false)?
    }

    Ok(())
}
