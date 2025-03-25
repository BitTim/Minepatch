/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 17:52
 */
use crate::bundle::BundleMessage;
use crate::bundle::data::BundleRepo;
use crate::patch::PatchRepo;
use crate::prelude::*;
use crate::{event, patch};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, name: &str, exist_only: bool) -> Result<()> {
    let bundle_options = BundleRepo::by_name_many(conn, Some(name), false)?;
    let bundle = event::select(
        tx,
        bundle_options,
        Message::Bundle(BundleMessage::Select),
        |bundle| Message::Bundle(BundleMessage::Option { bundle }),
    )?;

    if exist_only {
        return Ok(());
    }

    validate_patches(conn, tx, &bundle.name)?;
    Ok(())
}

fn validate_patches(conn: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    let patches = PatchRepo::by_name_many(conn, None, Some(name), true)?;

    for patch in patches {
        patch::validate(conn, tx, &patch.name, &patch.bundle, false)?
    }

    Ok(())
}
