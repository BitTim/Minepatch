/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:15
 */
use crate::db::Repo;
use crate::error::Error;
use crate::patch::{simulate, PatchError};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use rusqlite::Connection;

pub fn exclude(connection: &Connection, name: &str, pack: &str, mod_hash: &str) -> Result<()> {
    let query = PatchModRelFilter::ByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    let relation = PatchModRelRepo::query_single(connection, &query);

    let mods = simulate(connection, name, pack)?;
    if !mods.contains(&mod_hash.to_owned()) {
        return Err(Error::Patch(PatchError::ModExcluded(
            mod_hash.to_owned(),
            pack.to_owned(),
            name.to_owned(),
        )));
    }

    if let Ok(relation) = relation {
        if relation.removed {
            Err(Error::Patch(PatchError::RelTaken(
                mod_hash.to_owned(),
                name.to_owned(),
                pack.to_owned(),
            )))
        } else {
            PatchModRelRepo::remove(connection, &query)?;
            Ok(())
        }
    } else {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, mod_hash, true))?;
        Ok(())
    }
}
