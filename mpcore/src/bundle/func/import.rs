/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       import.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:25
 */
use crate::bundle::data::PortableBundle;
use crate::bundle::BundleProcess;
use crate::db::Portable;
use crate::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn import(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: Option<&str>,
) -> Result<()> {
    let portable = PortableBundle::import(tx, path, Process::Bundle(BundleProcess::Import), None)?;
    portable.insert(conn, name)
}
