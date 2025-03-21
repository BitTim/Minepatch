/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:51
 */
use crate::db::Repo;
use crate::patch::Patch;
use crate::patch::data::{PatchFilter, PatchRepo};
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
    let query = PatchFilter::ByNameAndBundleExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let patch = PatchRepo::by_filter(conn, &query)?;

    if exist_only {
        return Ok(());
    }

    bundle::validate(conn, tx, bundle, true)?;
    validate_patch_dependency(conn, tx, &patch)?;
    validate_mods(conn, name, bundle)?;
    Ok(())
}

fn validate_patch_dependency(conn: &Connection, tx: &Sender<Event>, patch: &Patch) -> Result<()> {
    if !patch.dependency.is_empty() {
        validate(conn, tx, &patch.dependency, &patch.bundle, false)?;
    }

    Ok(())
}

fn validate_mods(conn: &Connection, name: &str, bundle: &str) -> Result<()> {
    let query = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };

    PatchModRelRepo::by_filter_many(conn, &query)?
        .iter()
        .try_for_each(|value| vault::validate(conn, &value.mod_hash))?;

    Ok(())
}
