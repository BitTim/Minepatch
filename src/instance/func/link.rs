/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */
use crate::common::event;
use crate::common::event::Event;
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
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
    event::init_progress(tx, Process::Instance(InstanceProcess::Link), None)?;
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

    // FIXME: Introduce error if path is already present in DB

    let (patch, pack) = instance::detect(connection, tx, path, pack)?;
    let instance = Instance::new(actual_name, path, &pack, &patch);

    InstanceRepo::insert(connection, instance.clone())?;
    validate(connection, tx, actual_name, false)?;

    instance::apply(connection, tx, actual_name, &patch)?;

    event::end_progress(
        tx,
        Process::Instance(InstanceProcess::Link),
        Some(Message::Instance(InstanceMessage::LinkSuccess {
            instance: Box::new(instance),
        })),
    )?;
    Ok(actual_name.to_owned())
}
