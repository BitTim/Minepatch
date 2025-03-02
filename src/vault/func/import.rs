/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 16:25
 */
use crate::db::Repo;
use crate::file::build_vault_path;
use crate::prelude::*;
use crate::vault::data::VaultRepo;
use crate::vault::{Mod, ModMessage, ModProcess, PortableMod};
use crate::{comp, event};
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
    let deserialized = bincode::deserialize::<PortableMod>(&decompressed)?;

    let new_vault_path = build_vault_path(
        &deserialized.meta.id,
        &deserialized.meta.loader,
        &deserialized.filename,
    )?;

    let mut jar = File::create_new(&new_vault_path)?;
    jar.write_all(&deserialized.jar_binary)?;

    let hash = deserialized.hash.to_owned();
    VaultRepo::insert(conn, Mod::new(&hash, &new_vault_path, deserialized.meta))?;

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
