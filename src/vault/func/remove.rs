/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.02.25, 03:16
 */
use crate::common::msg;
use crate::db::Repo;
use crate::file;
use crate::prelude::*;
use crate::vault::data::{Mod, ModFilter, VaultRepo};
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use crate::vault::{ModMessage, ModProcess};
use rusqlite::Connection;
use std::collections::HashSet;
use std::fs;
use std::sync::mpsc::Sender;

pub fn remove<F>(
    connection: &Connection,
    tx: &Sender<Event>,
    hash: Option<&String>,
    all: bool,
    yes: bool,
    resolve: F,
) -> Result<()>
where
    F: Fn(&HashSet<Mod>) -> Result<&Mod>,
{
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

    msg::init_progress(
        tx,
        Process::Mod(ModProcess::Remove),
        Some(hashes.len() as u64),
    )?;
    for hash in hashes {
        let query = ModFilter::QueryHashExact {
            hash: hash.to_owned(),
        };
        let matches = VaultRepo::query_multiple(connection, &query)?;
        if matches.is_empty() {
            return Err(Error::Vault(VaultError::NotFound { hash }));
        }

        // TMP
        let value = matches.into_iter().next().unwrap();

        // TODO: Implement Event::Select
        //let value = resolve(&matches)?;
        if !yes
            && !msg::confirm(
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

        msg::tick_progress(
            tx,
            Process::Mod(ModProcess::Remove),
            Message::Mod(ModMessage::RemoveStatus {
                hash: hash.to_owned(),
            }),
        )?;
    }

    msg::end_progress(tx, Process::Mod(ModProcess::Remove), None)?;
    Ok(())
}
