/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 10:48
 */
use crate::bundle;
use crate::common::event;
use crate::common::event::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{Patch, PatchError, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelRepo, PatchModRelation};
use crate::prelude::*;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn create(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    dependency: &str,
    added: &HashSet<String>,
    removed: &HashSet<String>,
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Create), None)?;

    let exists_query = PatchFilter::ByNameAndBundleExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    if PatchRepo::exists(conn, &exists_query)? {
        return Err(Error::Patch(PatchError::NameExists {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
        }));
    }

    bundle::validate(conn, tx, bundle, true)?;
    let patch = Patch::new(name, bundle, dependency);
    PatchRepo::insert(conn, patch.to_owned())?;

    for hash in added {
        PatchModRelRepo::insert(conn, PatchModRelation::new(name, bundle, hash, false))?;
    }

    for hash in removed {
        PatchModRelRepo::insert(conn, PatchModRelation::new(name, bundle, hash, true))?;
    }

    event::end_progress(
        tx,
        Process::Patch(PatchProcess::Create),
        Some(Message::Patch(PatchMessage::CreateSuccess {
            patch: Box::new(patch),
        })),
    )?;
    Ok(())
}
