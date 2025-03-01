/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       export.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 00:12
 */
use crate::prelude::*;
use crate::vault::data::PortableMod;
use crate::vault::{ModMessage, ModProcess};
use crate::{comp, event, file, vault};
use rusqlite::Connection;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn export(conn: &Connection, tx: &Sender<Event>, hash: &str, path: &Path) -> Result<()> {
    event::init_progress(tx, Process::Mod(ModProcess::Export), None)?;

    let entity = vault::query_single_loose_hash(conn, hash)?;
    let portable = PortableMod::new(entity)?;

    let path = file::canonicalize_entity_path(path.to_owned(), &portable)?;
    let serialized = bincode::serialize(&portable)?;
    let compressed = comp::compress(&serialized)?;

    let mut file = File::create_new(&path)?;
    file.write_all(&compressed)?;

    event::end_progress(
        tx,
        Process::Mod(ModProcess::Export),
        Some(Message::Mod(ModMessage::ExportSuccess {
            hash: hash.to_owned(),
            path: path.to_owned(),
        })),
    )?;
    Ok(())
}
