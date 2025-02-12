/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:26
 */
use crate::common::msg;
use crate::common::msg::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::patch;
use crate::patch::{PatchError, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

// TODO: Exclude mod by query instead of by hash (User can specify a name and select from a list)
pub fn exclude(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    msg::init_progress(tx, Process::Patch(PatchProcess::Exclude), None)?;
    let query = PatchModRelFilter::ByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    let relation = PatchModRelRepo::query_single(connection, &query);

    let mods = patch::simulate(connection, tx, name, pack)?;
    if !mods.contains(&mod_hash.to_owned()) {
        return Err(Error::Patch(PatchError::ModExcluded {
            hash: mod_hash.to_owned(),
            pack: pack.to_owned(),
            name: name.to_owned(),
        }));
    }

    if let Ok(relation) = relation {
        if relation.removed {
            return Err(Error::Patch(PatchError::RelTaken {
                hash: mod_hash.to_owned(),
                name: name.to_owned(),
                pack: pack.to_owned(),
            }));
        } else {
            PatchModRelRepo::remove(connection, &query)?;
        }
    } else {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, mod_hash, true))?;
    }

    msg::end_progress(
        tx,
        Process::Patch(PatchProcess::Exclude),
        Some(Message::Patch(PatchMessage::ExcludeSuccess {
            hash: mod_hash.to_owned(),
        })),
    )?;
    Ok(())
}
