/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       repo.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 18:57
 */
use crate::error::Error;
use crate::patch_with_mods::data::model::PatchWithMods;
use crate::prelude;
use rusqlite::{params, Connection};

pub(crate) fn query_by_patch(
    connection: &Connection,
    patch: &str,
    pack: &str,
) -> prelude::Result<Vec<PatchWithMods>> {
    let mut statement =
        connection.prepare("SELECT * FROM patch_with_mods WHERE patch = ?1 AND pack = ?2")?;

    let raw_results = statement.query_and_then(params![patch, pack], |row| {
        let mod_hash: String = row.get(2)?;
        Ok::<PatchWithMods, Error>(PatchWithMods::new(patch, pack, &mod_hash, row.get(3)?))
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results)
}

pub(crate) fn query(
    connection: &Connection,
    patch: &str,
    pack: &str,
    mod_hash: &str,
) -> prelude::Result<Option<bool>> {
    let mut statement = connection
        .prepare("SELECT * FROM patch_with_mods WHERE patch = ?1 AND pack = ?2 AND mod = ?3")?;

    let raw_results = statement.query_and_then(params![patch, pack, mod_hash], |row| {
        Ok::<bool, Error>(row.get(3)?)
    })?;

    let mut results = vec![];
    for result in raw_results {
        results.push(result?);
    }

    Ok(results.first().copied())
}

pub(crate) fn insert(
    connection: &Connection,
    patch_with_mods: PatchWithMods,
) -> prelude::Result<i64> {
    let mut statement = connection.prepare(
        "INSERT INTO patch_with_mods (patch, pack, mod, removed) VALUES (?1, ?2, ?3, ?4)",
    )?;
    Ok(statement.insert(params![
        patch_with_mods.patch,
        patch_with_mods.pack,
        patch_with_mods.mod_hash,
        patch_with_mods.removed
    ])?)
}
