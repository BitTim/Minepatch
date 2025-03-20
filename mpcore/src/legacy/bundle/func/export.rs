/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       export.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Portable;
use crate::legacy::bundle::data::PortableBundle;
use crate::legacy::bundle::{BundleMessage, BundleProcess};
use crate::legacy::msg::Process;
use crate::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn export(conn: &Connection, tx: &Sender<Event>, name: &str, path: &Path) -> Result<()> {
    PortableBundle::new(conn, name)?.export(
        tx,
        path,
        Process::Bundle(BundleProcess::Export),
        Some(Message::Bundle(BundleMessage::ExportSuccess {
            bundle: name.to_owned(),
            path: path.to_owned(),
        })),
    )
}
