/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       export.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:23
 */
use crate::event::Event;
use crate::prelude::*;
use crate::template::{PortableTemplate, TemplateMessage, TemplateProcess};
use crate::{comp, event, file, template};
use rusqlite::Connection;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn export(conn: &Connection, tx: &Sender<Event>, name: &str, path: &Path) -> Result<()> {
    event::init_progress(tx, Process::Template(TemplateProcess::Export), None)?;

    let template = template::query_single(conn, name)?;
    let portable = PortableTemplate::new(template);

    let path = file::canonicalize_entity_path::<PortableTemplate>(path.to_owned(), &portable)?;
    let serialized = bincode::serialize(&portable)?;
    let compressed = comp::compress(&serialized)?;

    let mut file = File::create_new(&path)?;
    file.write_all(&compressed)?;

    event::end_progress(
        tx,
        Process::Template(TemplateProcess::Export),
        Some(Message::Template(TemplateMessage::ExportSuccess {
            name: name.to_owned(),
            path: path.to_owned(),
        })),
    )?;
    Ok(())
}
