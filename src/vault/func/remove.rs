/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 18:56
 */
use crate::common::event;
use crate::common::event::EventError;
use crate::db::Repo;
use crate::file;
use crate::prelude::*;
use crate::vault::data::{Mod, ModFilter, VaultRepo};
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use crate::vault::{ModMessage, ModProcess};
use rusqlite::Connection;
use std::fs;
use std::sync::mpsc::Sender;

pub fn remove(
    connection: &Connection,
    tx: &Sender<Event>,
    hash: Option<&String>,
    all: bool,
    yes: bool,
) -> Result<()> {
    // FIXME: Check if used before removing
    let hashes: Vec<String> = if all {
        let query_all = ModFilter::QueryAll;
        VaultRepo::query_multiple(connection, &query_all)?
            .iter()
            .map(|entry: &Mod| entry.hash.to_owned())
            .collect()
    } else {
        match hash {
            Some(hash) => vec![hash.to_owned()],
            None => return Err(Error::Vault(VaultError::NoHashProvided)),
        }
    };

    event::init_progress(
        tx,
        Process::Mod(ModProcess::Remove),
        Some(hashes.len() as u64),
    )?;
    for hash in hashes {
        let query = ModFilter::QueryHashAndIDAndNameSimilar {
            hash: hash.to_owned(),
            mod_id: "".to_string(),
            name: "".to_string(),
        };
        let matches = VaultRepo::query_multiple(connection, &query)?;
        if matches.is_empty() {
            return Err(Error::Vault(VaultError::NotFound { hash }));
        }

        let values = event::select(
            tx,
            matches.into_iter().collect(),
            Message::Mod(ModMessage::RemoveSelect),
            false,
            |option| {
                Message::Mod(ModMessage::RemoveOption {
                    value: Box::new(option),
                })
            },
        )?;
        let value = values
            .into_iter()
            .next()
            .ok_or(Error::Event(EventError::InvalidSelection))?;

        if !yes
            && !event::confirm(
                tx,
                Message::Mod(ModMessage::RemoveConfirm {
                    value: Box::new(value.to_owned()),
                }),
            )?
        {
            continue;
        }

        file::check_exists(&value.path)?;
        fs::remove_file(&value.path)?;
        file::remove_empty_dirs(&get_base_mod_dir_path()?)?;

        let remove_query = ModFilter::QueryHashExact {
            hash: value.hash.to_owned(),
        };
        VaultRepo::remove(connection, &remove_query)?;

        event::tick_progress(
            tx,
            Process::Mod(ModProcess::Remove),
            Message::Mod(ModMessage::RemoveStatus {
                hash: hash.to_owned(),
            }),
        )?;
    }

    event::end_progress(tx, Process::Mod(ModProcess::Remove), None)?;
    Ok(())
}
