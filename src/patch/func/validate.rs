/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.02.25, 01:46
 */
use crate::common::event;
use crate::db::Repo;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{Patch, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use crate::{pack, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    exist_only: bool,
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Patch(PatchProcess::Validate),
        Message::Patch(PatchMessage::ValidateStatus {
            pack: pack.to_owned(),
            name: name.to_owned(),
        }),
    )?;

    let query = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = PatchRepo::query_single(connection, &query)?;

    if exist_only {
        event::end_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
        return Ok(());
    }

    pack::validate(connection, tx, pack, true)?;
    validate_patch_dependency(connection, tx, &patch)?;
    validate_mods(connection, tx, name, pack)?;

    event::end_progress(tx, Process::Patch(PatchProcess::Validate), None)?;
    Ok(())
}

fn validate_patch_dependency(
    connection: &Connection,
    tx: &Sender<Event>,
    patch: &Patch,
) -> Result<()> {
    if !patch.dependency.is_empty() {
        validate(connection, tx, &patch.dependency, &patch.pack, false)?;
    }

    Ok(())
}

fn validate_mods(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
) -> Result<()> {
    let query = PatchModRelFilter::ByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };

    PatchModRelRepo::query_multiple(connection, &query)?
        .iter()
        .try_for_each(|value| vault::validate(connection, tx, &value.mod_hash))?;

    Ok(())
}
