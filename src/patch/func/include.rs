/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 22:10
 */

use crate::common::Repo;
use crate::patch::PatchError;
use crate::patch_with_mods::{PatchModRelQueries, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use rusqlite::Connection;

pub fn include(connection: &Connection, name: &str, pack: &str, mod_hash: &str) -> Result<()> {
    let query = PatchModRelQueries::QueryByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    let relation = PatchModRelRepo::query_single(connection, &query);

    if let Ok(relation) = relation {
        if relation.removed {
            PatchModRelRepo::remove(connection, &query)?;
            Ok(())
        } else {
            Err(Error::Patch(PatchError::ModAlreadyIncluded(
                mod_hash.to_owned(),
                name.to_owned(),
                pack.to_owned(),
            )))
        }
    } else {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, mod_hash, false))?;
        Ok(())
    }
}
