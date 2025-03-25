/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 17:51
 */
use crate::common::event;
use crate::db::Repo;
use crate::file;
use crate::file::get_base_vault_path;
use crate::patch_with_mods::{PatchModRelFilter, PatchModRelRepo};
use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::data::{VaultFilter, VaultRepo};
use crate::vault::error::VaultError;
use crate::vault::{ModMessage, ModProcess};
use rusqlite::Connection;
use std::fs;
use std::sync::mpsc::Sender;

pub fn remove(
    conn: &Connection,
    tx: &Sender<Event>,
    hash: Option<&String>,
    all: bool,
    yes: bool,
) -> Result<()> {
    let hashes: Vec<String> = if all {
        VaultRepo::by_hash_many(conn, None, false)?
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
        let query = VaultFilter::ByHashAndIDAndNameSimilar {
            hash: hash.to_owned(),
            mod_id: "".to_string(),
            name: "".to_string(),
        };
        let matches = VaultRepo::by_filter_many(conn, &query)?;
        if matches.is_empty() {
            return Err(Error::Vault(VaultError::NotFound { hash }));
        }

        let value = event::select(
            tx,
            matches.into_iter().collect(),
            Message::Mod(ModMessage::RemoveSelect),
            |option| {
                Message::Mod(ModMessage::RemoveOption {
                    value: Box::new(option),
                })
            },
        )?;

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

        let rel_filter = PatchModRelFilter::ByModHashExact {
            hash: value.hash.to_owned(),
        };
        let relations = PatchModRelRepo::by_filter_many(conn, &rel_filter)?;

        if !relations.is_empty() {
            return Err(Error::Vault(VaultError::RelUsed {
                hash: value.hash.to_owned(),
            }));
        }

        let remove_filter = VaultFilter::ByHashExact {
            hash: value.hash.to_owned(),
        };
        VaultRepo::remove(conn, &remove_filter)?;

        file::check_exists(&value.path)?;
        fs::remove_file(&value.path)?;
        file::remove_empty_dirs(&get_base_vault_path()?)?;

        event::tick_progress(
            tx,
            Process::Mod(ModProcess::Remove),
            Message::Mod(ModMessage::RemoveStatus {
                hash: hash.to_owned(),
            }),
            1,
        )?;
    }

    event::end_progress(tx, Process::Mod(ModProcess::Remove), None)
}
