/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:19
 */

use crate::common::Repo;
use crate::patch::data::{PatchQueries, PatchRepo};
use crate::patch_with_mods;
use crate::prelude::*;
use rusqlite::Connection;

pub fn simulate(connection: &Connection, name: &str, pack: &str) -> Result<Vec<String>> {
    let query = PatchQueries::QueryNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = PatchRepo::query_single(connection, query)?;

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
