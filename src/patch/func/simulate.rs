/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:11
 */

use crate::common::Repo;
use crate::patch::data::{PatchQueries, PatchRepo};
use crate::patch_with_mods::{PatchModRelQueries, PatchModRelRepo};
use crate::prelude::*;
use rusqlite::Connection;

pub fn simulate(connection: &Connection, name: &str, pack: &str) -> Result<Vec<String>> {
    let patch_query = PatchQueries::QueryNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    let patch = PatchRepo::query_single(connection, &patch_query)?;

    let mut mod_hashes = vec![];
    if !patch.dependency.is_empty() {
        mod_hashes.append(&mut simulate(connection, &patch.dependency, pack)?);
    }

    let rel_query = PatchModRelQueries::QueryByPatchAndPackExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
    };
    let mod_relations = PatchModRelRepo::query_multiple(connection, &rel_query)?;
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
