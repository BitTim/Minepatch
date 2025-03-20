/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       clean.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 14:26
 */
use crate::db::Repo;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use crate::vault::{ModMessage, ModProcess};
use crate::{event, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn clean(conn: &Connection, tx: &Sender<Event>) -> Result<()> {
    let hashes = vault::query_multiple(conn, None, None, None)?
        .iter()
        .map(|value| (value.hash.to_owned(), value.meta.id.to_owned()))
        .collect::<Vec<(String, String)>>();

    event::init_progress(
        tx,
        Process::Mod(ModProcess::Clean),
        Some(hashes.len() as u64),
    )?;
    let mut deleted: Vec<(String, String)> = vec![];

    for (hash, id) in hashes {
        event::tick_progress(
            tx,
            Process::Mod(ModProcess::Clean),
            Message::Mod(ModMessage::CleanStatus {
                hash: hash.to_owned(),
                id: id.to_owned(),
            }),
            1,
        )?;

        let rel_filter = PatchModRelFilter::ByModHashExact {
            hash: hash.to_owned(),
        };
        if !PatchModRelRepo::exists(conn, &rel_filter)? {
            deleted.push((hash.to_owned(), id));
            vault::remove(conn, tx, Some(&hash), false, true)?
        }
    }

    event::end_progress(
        tx,
        Process::Mod(ModProcess::Clean),
        Some(Message::Mod(ModMessage::CleanSuccess { values: deleted })),
    )?;
    Ok(())
}
