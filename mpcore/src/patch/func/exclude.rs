/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       exclude.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:26
 */
use crate::common::event;
use crate::common::event::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::patch;
use crate::patch::{PatchError, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo, PatchModRelation};
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
