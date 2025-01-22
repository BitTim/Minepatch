/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 19:07
 */
use crate::patch::{data, PatchError};
use crate::patch_with_mods;
use crate::prelude::*;
use rusqlite::Connection;

pub fn simulate(connection: &Connection, name: &str, pack: &str) -> Result<Vec<String>> {
    let results = data::query(connection, name, pack)?;
    let patch = results
        .first()
        .ok_or_else(|| Error::Patch(PatchError::NotFound(name.to_owned(), pack.to_owned())))?;

    let mut mod_hashes = vec![];
    if !patch.dependency.is_empty() {
        mod_hashes.append(&mut simulate(connection, &patch.dependency, pack)?);
    }

    let mod_relations = patch_with_mods::query_by_patch(connection, name, pack)?;
    for relation in mod_relations {
        let result = mod_hashes
            .iter()
            .enumerate()
            .find(|(_, hash)| **hash == relation.mod_hash);

        match relation.removed {
            true => match result {
                None => continue,
                Some((index, _)) => _ = mod_hashes.swap_remove(index),
            },
            false => match result {
                None => mod_hashes.push(relation.mod_hash),
                Some(_) => continue,
            },
        }
    }

    Ok(mod_hashes)
}
