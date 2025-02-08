/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   07.02.25, 17:23
 */
use crate::db::Repo;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::Patch;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use crate::{pack, vault};
use rusqlite::Connection;

pub fn validate(connection: &Connection, name: &str, pack: &str, exist_only: bool) -> Result<()> {
    let query = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = PatchRepo::query_single(connection, &query)?;

    if exist_only {
        return Ok(());
    }

    pack::validate(connection, pack, true)?;
    validate_patch_dependency(connection, &patch)?;
    validate_mods(connection, name, pack)?;

    Ok(())
}

fn validate_patch_dependency(connection: &Connection, patch: &Patch) -> Result<()> {
    if !patch.dependency.is_empty() {
        validate(connection, &patch.dependency, &patch.pack, false)?;
    }

    Ok(())
}

fn validate_mods(connection: &Connection, name: &str, pack: &str) -> Result<()> {
    let query = PatchModRelFilter::QueryByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };

    PatchModRelRepo::query_multiple(connection, &query)?
        .iter()
        .map(|value| vault::validate(connection, &value.mod_hash))
        .collect::<Result<()>>()?;

    Ok(())
}
