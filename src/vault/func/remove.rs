/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:55
 */
use crate::file;
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use rusqlite::Connection;
use std::fs;

pub fn remove<F>(
    connection: &Connection,
    hash: Option<&String>,
    all: bool,
    yes: bool,
    confirm: F,
) -> Result<()>
where
    F: Fn(&Mod) -> Result<bool>,
{
    let hashes: Vec<String> = if all {
        data::query(connection, "", "", "")?
            .iter()
            .map(|entry: &Mod| entry.hash.to_owned())
            .collect()
    } else {
        match hash {
            Some(hash) => vec![hash.to_owned()],
            None => return Err(Error::Vault(VaultError::NotFound("".to_owned()))),
        }
    };

    for hash in hashes {
        let matches = data::query(connection, &hash, "", "")?;
        if matches.is_empty() {
            return Err(Error::Vault(VaultError::NotFound(hash)));
        }

        for entry in matches {
            if !yes && confirm(&entry)? {
                continue;
            }

            file::check_exists(&entry.path)?;
            fs::remove_file(&entry.path)?;
            file::remove_empty_dirs(&get_base_mod_dir_path()?)?;

            data::remove(connection, &entry.hash)?;
        }
    }

    Ok(())
}
