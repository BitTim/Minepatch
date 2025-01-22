/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 03:20
 */

use crate::patch::{insert_relation, query_relation, PatchError};
use crate::prelude::*;
use rusqlite::Connection;

pub fn include(connection: &Connection, name: &str, pack: &str, mod_hash: &str) -> Result<()> {
    let relation = query_relation(connection, name, pack, mod_hash)?;

    match relation {
        None => Ok(_ = insert_relation(connection, name, pack, mod_hash, false)?),
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
