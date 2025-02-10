/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       include.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:51
 */
use crate::db::Repo;
use crate::patch;
use crate::patch::PatchError;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
use crate::progress::event::Event;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn include(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
    mod_hash: &str,
) -> Result<()> {
    let rel_filter = PatchModRelFilter::ByPatchAndPackAndModHashExact {
        patch: name.to_owned(),
        pack: pack.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    let relation = PatchModRelRepo::query_single(connection, &rel_filter);

    let mods = patch::simulate(connection, tx, name, pack)?;
    if mods.contains(mod_hash) {
        return Err(Error::Patch(PatchError::ModIncluded {
            hash: mod_hash.to_owned(),
            pack: pack.to_owned(),
            name: name.to_owned(),
        }));
    }

    if let Ok(relation) = relation {
        if relation.removed {
            PatchModRelRepo::remove(connection, &rel_filter)?;
            Ok(())
        } else {
            Err(Error::Patch(PatchError::RelTaken {
                hash: mod_hash.to_owned(),
                name: name.to_owned(),
                pack: pack.to_owned(),
            }))
        }
    } else {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, mod_hash, false))?;
        Ok(())
    }
}
