/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 19:07
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::db::Repo;
use crate::instance;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{validate, Instance, InstanceError};
use crate::msg::Message;
use crate::prelude::*;
use crate::progress::event::Event;
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

    let (patch, pack) = instance::detect(connection, tx, path, pack)?;

    InstanceRepo::insert(connection, Instance::new(actual_name, path, &pack, &patch))?;
    validate(connection, tx, actual_name, false)?;

    instance::apply(connection, tx, actual_name, &patch)?;

    tx.send(Event::Success {
        message: Message::new("Linked instance")
            .context("Name", actual_name)
            .context("Path", &path.display().to_string())
            .context("Pack", &pack)
            .context("Patch", &patch),
    })?;
    Ok(actual_name.to_owned())
}
