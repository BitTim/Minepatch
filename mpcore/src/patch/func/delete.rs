/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       delete.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:54
 */
use crate::db::Repo;
use crate::event;
use crate::event::Event;
use crate::instance::data::InstanceRepo;
use crate::patch::{PatchError, PatchFilter, PatchMessage, PatchProcess, PatchRepo};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn delete(conn: &Connection, tx: &Sender<Event>, name: &str, bundle: &str) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Delete), None)?;

    let filter = PatchFilter::ByNameAndBundleExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };

    if !PatchRepo::exists_by_filter(conn, &filter)? {
        return Err(Error::Patch(PatchError::NotFound {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
        }));
    }

    // TODO: Handle "exact" value
    let dependant = PatchRepo::by_dependency(conn, name, bundle, true);
    if dependant.is_ok() {
        return Err(Error::Patch(PatchError::PatchInUseByPatch {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
            dependant: dependant?.name,
        }));
    }

    let instance = InstanceRepo::by_patch(conn, name, true);
    if instance.is_ok() {
        return Err(Error::Patch(PatchError::PatchInUseByInstance {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
            instance: instance?.name.to_owned(),
        }));
    }

    let rel_filter = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    PatchModRelRepo::remove(conn, &rel_filter)?;
    PatchRepo::remove(conn, &filter)?;

    event::end_progress(
        tx,
        Process::Patch(PatchProcess::Delete),
        Some(Message::Patch(PatchMessage::DeleteSuccess {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
        })),
    )?;
    Ok(())
}
