/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 17:39
 */
use crate::common::msg::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::pack;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{Patch, PatchError};
use crate::patch_with_mods::{PatchModRelRepo, PatchWithMods};
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    dependency: &str,
    added: &HashSet<String>,
    removed: &HashSet<String>,
) -> crate::prelude::Result<()> {
    let exists_query = PatchFilter::ByNameAndPackExact {
        name: name.to_owned(),
        pack: pack.to_owned(),
    };
    if PatchRepo::exists(connection, &exists_query)? {
        return Err(Error::Patch(PatchError::NameExists {
            name: name.to_owned(),
            pack: pack.to_owned(),
        }));
    }

    pack::validate(connection, pack, true)?;
    PatchRepo::insert(connection, Patch::new(name, pack, dependency))?;

    for hash in added {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, hash, false))?;
    }

    for hash in removed {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, hash, true))?;
    }

    Ok(())
}
