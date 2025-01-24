/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.01.25, 16:50
 */

use crate::patch::PatchError;
use crate::patch_with_mods;
use crate::patch_with_mods::PatchWithMods;
use crate::prelude::*;
use rusqlite::Connection;

pub fn include(connection: &Connection, name: &str, pack: &str, mod_hash: &str) -> Result<()> {
    let relation = patch_with_mods::query(connection, name, pack, mod_hash)?;

    match relation {
        None => Ok(_ =
            patch_with_mods::insert(connection, PatchWithMods::new(name, pack, mod_hash, false))?),
        Some(removed) => match removed {
            true => Ok(()), // TODO: Add update method
            false => Err(Error::Patch(PatchError::ModAlreadyIncluded(
                mod_hash.to_owned(),
                name.to_owned(),
                pack.to_owned(),
            ))),
        },
    }
}
