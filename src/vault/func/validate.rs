/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event;
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::{ModFilter, VaultRepo};
use crate::vault::{ModMessage, ModProcess, VaultError};
use rusqlite::Connection;
use std::fs;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, hash: &str) -> Result<()> {
    event::init_progress(tx, Process::Mod(ModProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Mod(ModProcess::Validate),
        Message::Mod(ModMessage::ValidateStatus {
            hash: hash.to_owned(),
        }),
    )?;

    let query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };
    let value = VaultRepo::query_single(conn, &query)?;

    if !fs::exists(&value.path)? {
        return Err(Error::Vault(VaultError::PathNotExist {
            hash: hash.to_owned(),
            path: value.path.display().to_string(),
        }));
    }

    event::end_progress(tx, Process::Mod(ModProcess::Validate), None)?;
    Ok(())
}
