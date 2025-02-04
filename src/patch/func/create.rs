/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 19:15
 */
use crate::common::Repo;
use crate::error::Error;
use crate::pack;
use crate::pack::PackError;
use crate::patch::data::{PatchQueries, PatchRepo};
use crate::patch::{Patch, PatchError};
use rusqlite::Connection;

pub fn create(
    connection: &Connection,
    name: &str,
    pack: &str,
    dependency: &str,
    state_hash: &str,
) -> crate::prelude::Result<()> {
    let exists_query = PatchQueries::QueryNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    if PatchRepo::exists(connection, exists_query)? {
        return Err(Error::Patch(PatchError::NameExists(
            name.to_owned(),
            pack.to_owned(),
        )));
    }

    if !pack::validate(connection, pack, true) {
        return Err(Error::Pack(PackError::NotFound(pack.to_owned())));
    }

    PatchRepo::insert(connection, Patch::new(name, pack, dependency, state_hash))?;
    Ok(())
}
