/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */
use crate::common::event;
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::{ModFilter, VaultRepo};
use crate::vault::{ModMessage, ModProcess, VaultError};
use rusqlite::Connection;
use std::fs;
use std::sync::mpsc::Sender;

pub fn validate(connection: &Connection, tx: &Sender<Event>, hash: &str) -> Result<()> {
    event::init_progress(tx, Process::Mod(ModProcess::Validate), None)?;

    let query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };
    let value = VaultRepo::query_single(connection, &query)?;

    if !fs::exists(&value.path)? {
        return Err(Error::Vault(VaultError::PathNotExist {
            hash: hash.to_owned(),
            path: value.path.display().to_string(),
        }));
    }

    event::end_progress(
        tx,
        Process::Mod(ModProcess::Validate),
        Some(Message::Mod(ModMessage::ValidateSuccess {
            hash: hash.to_owned(),
        })),
    )?;
    Ok(())
}
