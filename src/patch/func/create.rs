/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */
use crate::common::event;
use crate::common::event::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::pack;
use crate::patch::data::{PatchFilter, PatchRepo};
use crate::patch::{Patch, PatchError, PatchMessage, PatchProcess};
use crate::patch_with_mods::{PatchModRelRepo, PatchWithMods};
use crate::prelude::*;
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
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Create), None)?;

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

    pack::validate(connection, tx, pack, true)?;
    let patch = Patch::new(name, pack, dependency);
    PatchRepo::insert(connection, patch.to_owned())?;

    for hash in added {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, hash, false))?;
    }

    for hash in removed {
        PatchModRelRepo::insert(connection, PatchWithMods::new(name, pack, hash, true))?;
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
