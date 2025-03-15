/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       rename.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 12:39
 */
use crate::db::Repo;
use crate::event;
use crate::msg::Process;
use crate::patch::{PatchFilter, PatchMessage, PatchProcess, PatchRepo};
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
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
    PatchRepo::insert(conn, patch)?;

    let rel_filter = PatchModRelFilter::ByPatchAndBundleExact {
        patch: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let relations = PatchModRelRepo::query_multiple(conn, &rel_filter)?;
    for mut rel in relations {
        rel.patch = new_name.to_owned();
        let filter = PatchModRelFilter::ByPatchAndBundleAndModHashExact {
            patch: name.to_owned(),
            bundle: bundle.to_owned(),
            mod_hash: rel.mod_hash.to_owned(),
        };
        PatchModRelRepo::update(conn, &filter, rel)?;
    }

    PatchRepo::remove(conn, &filter)?;

    let dep_filter = PatchFilter::ByDepAndBundleExact {
        dependency: name.to_owned(),
        bundle: bundle.to_owned(),
    };
    let dependants = PatchRepo::query_multiple(conn, &dep_filter)?;
    for mut dep in dependants {
        dep.dependency = new_name.to_owned();
        let filter = PatchFilter::ByNameAndBundleExact {
            name: dep.name.to_owned(),
            bundle: bundle.to_owned(),
        };
        PatchRepo::update(conn, &filter, dep)?;
    }

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
