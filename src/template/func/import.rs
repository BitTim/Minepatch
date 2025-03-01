/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 18:27
 */
use crate::db::Repo;
use crate::event::Event;
use crate::prelude::*;
use crate::template::data::TemplateRepo;
use crate::template::{Template, TemplateMessage, TemplateProcess};
use crate::{comp, event};
use rusqlite::Connection;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn import(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: Option<&str>,
) -> Result<()> {
    event::init_progress(tx, Process::Template(TemplateProcess::Import), None)?;

    let mut file = File::open(path)?;
    let mut data: Vec<u8> = vec![];

    file.read_to_end(&mut data)?;

    let decompressed = comp::decompress(&data)?;
    let mut deserialized = bincode::deserialize::<Template>(&decompressed)?;

    if name.is_some() {
        deserialized.name = name.unwrap().to_owned();
    }

    let imported_name = deserialized.name.to_owned();
    TemplateRepo::insert(conn, deserialized)?;

    event::end_progress(
        tx,
        Process::Template(TemplateProcess::Import),
        Some(Message::Template(TemplateMessage::ImportSuccess {
            name: imported_name,
            path: path.to_owned(),
        })),
    )?;
    Ok(())
}
