/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       rename.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 11:47
 */
use crate::db::Repo;
use crate::event;
use crate::msg::Process;
use crate::patch::{PatchFilter, PatchMessage, PatchProcess, PatchRepo};
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn rename(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    new_name: &str,
) -> Result<()> {
    event::init_progress(tx, Process::Patch(PatchProcess::Rename), None)?;

    let filter = PatchFilter::ByNameAndBundleExact {
        name: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let mut patch = PatchRepo::query_single(conn, &filter)?;
    patch.name = new_name.to_owned();
    PatchRepo::update(conn, &filter, patch)?;

    event::end_progress(
        tx,
        Process::Patch(PatchProcess::Rename),
        Some(Message::Patch(PatchMessage::RenameSuccess {
            name: name.to_owned(),
            bundle: bundle.to_owned(),
            new_name: new_name.to_owned(),
        })),
    )?;
    Ok(())
}
