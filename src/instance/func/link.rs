/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:52
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::msg::Event;
use crate::db::Repo;
use crate::instance;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{validate, Instance, InstanceContext, InstanceError, InstanceMessage};
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
    let instance = Instance::new(actual_name, path, &pack, &patch);

    InstanceRepo::insert(connection, instance.clone())?;
    validate(connection, tx, actual_name, false)?;

    instance::apply(connection, tx, actual_name, &patch)?;

    tx.send(Event::Success {
        message: Message::Instance(InstanceMessage::LinkSuccess(vec![
            InstanceContext::SuccessObj(instance),
        ])),
    })?;
    Ok(actual_name.to_owned())
}
