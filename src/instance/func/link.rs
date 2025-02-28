/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:43
 */
use crate::common::event;
use crate::common::event::Event;
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::db::Repo;
use crate::instance;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{Instance, InstanceError, InstanceMessage, InstanceProcess, validate};
use crate::prelude::*;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn link(
    conn: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    name: Option<&str>,
    bundle: Option<&str>,
) -> Result<String> {
    event::init_progress(tx, Process::Instance(InstanceProcess::Link), None)?;
    if !fs::exists(path)? {
        return Err(Error::File(FileError::PathNotFound {
            path: path.to_owned(),
        }));
    }

    let actual_name = match name {
        Some(name) => name,
        None => filename_from_path(path)?,
    };

    let query = InstanceFilter::ByExactName {
        name: actual_name.to_owned(),
    };
    if InstanceRepo::exists(conn, &query)? {
        return Err(Error::Instance(InstanceError::NameTaken {
            name: actual_name.to_owned(),
        }));
    }

    // FIXME: Introduce error if path is already present in DB

    let (patch, bundle) = instance::detect(conn, tx, path, bundle)?;
    let instance = Instance::new(actual_name, path, &bundle, &patch);

    InstanceRepo::insert(conn, instance.clone())?;
    validate(conn, tx, actual_name, false)?;

    instance::apply(conn, tx, actual_name, &patch)?;

    event::end_progress(
        tx,
        Process::Instance(InstanceProcess::Link),
        Some(Message::Instance(InstanceMessage::LinkSuccess {
            instance: Box::new(instance),
        })),
    )?;
    Ok(actual_name.to_owned())
}
