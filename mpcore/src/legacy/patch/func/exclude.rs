/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Repo;
use crate::legacy::common::event;
use crate::legacy::common::event::Event;
use crate::legacy::error::Error;
use crate::legacy::patch;
use crate::legacy::patch::{PatchError, PatchMessage, PatchProcess};
use crate::legacy::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchModRelation};
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn exclude(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    mod_hash: &str,
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Exclude), None)?;
    let query = PatchModRelFilter::ByPatchAndBundleAndModHashExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
        mod_hash: mod_hash.to_owned(),
    };
    let relation = PatchModRelRepo::query_single(conn, &query);

    let mods = patch::simulate(conn, tx, name, bundle)?;
    if !mods.contains(mod_hash) {
        return Err(Error::Patch(PatchError::ModExcluded {
            hash: mod_hash.to_owned(),
            bundle: bundle.to_owned(),
            name: name.to_owned(),
        }));
    }

    if let Ok(relation) = relation {
        if relation.removed {
            return Err(Error::Patch(PatchError::RelTaken {
                hash: mod_hash.to_owned(),
                name: name.to_owned(),
                bundle: bundle.to_owned(),
            }));
        } else {
            PatchModRelRepo::remove(conn, &query)?;
        }
    } else {
        PatchModRelRepo::insert(conn, PatchModRelation::new(name, bundle, mod_hash, true))?;
    }

    event::end_progress(
        tx,
        Process::Patch(PatchProcess::Exclude),
        Some(Message::Patch(PatchMessage::ExcludeSuccess {
            hash: mod_hash.to_owned(),
        })),
    )?;
    Ok(())
}
