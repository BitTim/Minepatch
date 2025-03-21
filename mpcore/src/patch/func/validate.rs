/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 10:48
 */
use crate::common::event;
use crate::db::Repo;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{Patch, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use crate::{bundle, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    exist_only: bool,
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Patch(PatchProcess::Validate),
        Message::Patch(PatchMessage::ValidateStatus {
            bundle: bundle.to_owned(),
            name: name.to_owned(),
        }),
        1,
    )?;

    let query = PatchFilter::ByNameAndBundleExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let patch = PatchRepo::query_single(conn, &query)?;

    if exist_only {
        event::end_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
        return Ok(());
    }

    bundle::validate(conn, tx, bundle, true)?;
    validate_patch_dependency(conn, tx, &patch)?;
    validate_mods(conn, tx, name, bundle)?;

    event::end_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
    Ok(())
}

fn validate_patch_dependency(conn: &Connection, tx: &Sender<Event>, patch: &Patch) -> Result<()> {
    if !patch.dependency.is_empty() {
        validate(conn, tx, &patch.dependency, &patch.bundle, false)?;
    }

    Ok(())
}

fn validate_mods(conn: &Connection, tx: &Sender<Event>, name: &str, bundle: &str) -> Result<()> {
    let query = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };

    PatchModRelRepo::query_multiple(conn, &query)?
        .iter()
        .try_for_each(|value| vault::validate(conn, tx, &value.mod_hash))?;

    Ok(())
}
