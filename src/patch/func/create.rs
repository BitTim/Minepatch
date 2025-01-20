/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 21:58
 */
use crate::error::Error;
use crate::patch::{exists, insert, Patch, PatchError};
use rusqlite::Connection;

pub fn create(
    connection: &Connection,
    name: &str,
    dependency: &str,
    state_hash: &str,
    pack: &str,
) -> crate::prelude::Result<()> {
    if exists(connection, name, pack)? {
        return Err(Error::Patch(PatchError::NameExists(
            name.to_owned(),
            pack.to_owned(),
        )));
    }

    insert(connection, Patch::new(name, dependency, state_hash, pack))?;
    Ok(())
}
