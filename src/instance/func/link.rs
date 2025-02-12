/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 02:30
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::msg;
use crate::common::msg::Event;
use crate::db::Repo;
use crate::instance;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{validate, Instance, InstanceError, InstanceMessage, InstanceProcess};
use crate::prelude::*;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn link(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: Option<&str>,
    pack: Option<&str>,
) -> Result<String> {
    msg::init_progress(tx, Process::Instance(InstanceProcess::Link), None)?;
    if !fs::exists(path)? {
        return Err(Error::File(FileError::PathNotFound(
            path.display().to_string(),
        )));
    }

    let actual_name = match name {
        Some(name) => name,
        None => filename_from_path(path)?,
    };

    let query = InstanceFilter::ByExactName {
        name: actual_name.to_owned(),
    };
    if InstanceRepo::exists(connection, &query)? {
        return Err(Error::Instance(InstanceError::NameTaken {
            name: actual_name.to_owned(),
        }));
    }

    // FIXME: Error if path is already present in DB

    let (patch, pack) = instance::detect(connection, tx, path, pack)?;
    let instance = Instance::new(actual_name, path, &pack, &patch);

    InstanceRepo::insert(connection, instance.clone())?;
    validate(connection, tx, actual_name, false)?;

    instance::apply(connection, tx, actual_name, &patch)?;

    msg::end_progress(
        tx,
        Process::Instance(InstanceProcess::Link),
        Some(Message::Instance(InstanceMessage::LinkSuccess { instance })),
    )?;
    Ok(actual_name.to_owned())
}
