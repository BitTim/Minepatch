/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 16:42
 */
use crate::error::Error;
use crate::pack;
use crate::pack::PackError;
use crate::patch::{data, Patch, PatchError};
use rusqlite::Connection;

pub fn create(
    connection: &Connection,
    name: &str,
    dependency: &str,
    state_hash: &str,
    pack: &str,
) -> crate::prelude::Result<()> {
    if data::exists(connection, name, pack)? {
        return Err(Error::Patch(PatchError::NameExists(
            name.to_owned(),
            pack.to_owned(),
        )));
    }

    if !pack::data::exists(connection, pack)? {
        return Err(Error::Pack(PackError::NotFound(pack.to_owned())));
    }

    data::insert(connection, Patch::new(name, dependency, state_hash, pack))?;
    Ok(())
}
