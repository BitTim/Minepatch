/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */

use crate::common::event::Event;
use crate::common::{event, file, hash};
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::{Mod, ModFilter, VaultRepo};
use crate::vault::func::common::meta::{detect_loader, extract_meta};
use crate::vault::{ModMessage, ModProcess, VaultError};
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn add(
    connection: &Connection,
    tx: &Sender<Event>,
    path: &Path,
    overwrite: bool,
) -> Result<String> {
    event::init_progress(tx, Process::Mod(ModProcess::Add), None)?;

    file::check_exists(path)?;
    let hash = hash::hash_file(path)?;

    let exists_query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };

    if VaultRepo::exists(connection, &exists_query)? && !overwrite {
        tx.send(Event::Warning {
            warning: Box::new(Error::Vault(VaultError::AlreadyExists {
                path: path.display().to_string(),
                hash: hash.to_owned(),
            })),
        })?;
        return Ok(hash);
    }

    let loader_result = detect_loader(path)?;
    if loader_result.is_none() {
        tx.send(Event::Warning {
            warning: Box::new(Error::Vault(VaultError::NoLoaderDetected {
                path: path.display().to_string(),
            })),
        })?;
    }

    let filename = file::filename_from_path(path)?;
    let (meta, mod_file_path) = extract_meta(loader_result, filename)?;

    fs::copy(path, &mod_file_path)?;
    let value = Mod::new(&hash, &mod_file_path, meta);
    VaultRepo::insert(connection, value.to_owned())?;

    event::end_progress(
        tx,
        Process::Mod(ModProcess::Add),
        Some(Message::Mod(ModMessage::AddSuccess {
            value: Box::new(value),
        })),
    )?;
    Ok(hash)
}
