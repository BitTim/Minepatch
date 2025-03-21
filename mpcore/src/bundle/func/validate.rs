/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:52
 */
use crate::bundle::data::BundleRepo;
use crate::patch;
use crate::patch::PatchRepo;
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, name: &str, exist_only: bool) -> Result<()> {
    //TODO: Handle "exact" value
    let bundle = BundleRepo::by_name(conn, name, true)?;

    if exist_only {
        return Ok(());
    }

    validate_patches(conn, tx, &bundle.name)?;
    Ok(())
}

fn validate_patches(conn: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    // TODO: Handle "exact" value
    let patches = PatchRepo::by_name_many(conn, None, Some(name), true)?;

    for patch in patches {
        patch::validate(conn, tx, &patch.name, &patch.bundle, false)?
    }

    Ok(())
}
