/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 00:17
 */
use crate::db::Repo;
use crate::file::build_vault_path;
use crate::prelude::*;
use crate::vault::data::VaultRepo;
use crate::vault::{ModMessage, ModProcess, PortableMod};
use crate::{comp, event, file};
use rusqlite::Connection;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn import(conn: &Connection, tx: &Sender<Event>, path: &Path) -> Result<()> {
    event::init_progress(tx, Process::Mod(ModProcess::Import), None)?;

    let mut file = File::open(path)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let decompressed = comp::decompress(&data)?;
    let mut deserialized = bincode::deserialize::<PortableMod>(&decompressed)?;

    let new_vault_path = build_vault_path(
        &deserialized.entity.meta.id,
        &deserialized.entity.meta.loader,
        file::filename_from_path(&deserialized.entity.path)?,
    )?;

    let mut jar = File::create_new(&new_vault_path)?;
    jar.write_all(&deserialized.jar_binary)?;
    deserialized.entity.path = new_vault_path;

    let hash = deserialized.entity.hash.to_owned();
    VaultRepo::insert(conn, deserialized.entity)?;

    event::end_progress(
        tx,
        Process::Mod(ModProcess::Import),
        Some(Message::Mod(ModMessage::ImportSuccess {
            hash: hash.to_owned(),
            path: path.to_owned(),
        })),
    )?;
    Ok(())
}
