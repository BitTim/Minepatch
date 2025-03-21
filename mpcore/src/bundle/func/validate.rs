/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 07:37
 */
use crate::bundle::data::BundleRepo;
use crate::bundle::{BundleMessage, BundleProcess};
use crate::common::event;
use crate::patch;
use crate::prelude::*;
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

    let bundle = BundleRepo::by_name(conn, name)?;

    if exist_only {
        event::end_progress(tx, Process::Bundle(BundleProcess::Validate), None)?;
        return Ok(());
    }

    validate_patches(conn, tx, &bundle.name)?;

    event::end_progress(tx, Process::Bundle(BundleProcess::Validate), None)?;
    Ok(())
}

fn validate_patches(conn: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    let patches = patch::query_multiple(conn, None, Some(name))?;

    for patch in patches {
        patch::validate(conn, tx, &patch.name, &patch.bundle, false)?
    }

    Ok(())
}
