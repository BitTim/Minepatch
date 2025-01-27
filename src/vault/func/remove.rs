/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 11:33
 */
use crate::file;
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use crate::vault::func::common::path::get_base_mod_dir_path;
use rusqlite::Connection;
use std::fs;

pub fn remove<F, G>(
    connection: &Connection,
    hash: Option<&String>,
    all: bool,
    yes: bool,
    confirm: F,
    resolve: G,
) -> Result<()>
where
    F: Fn(&Mod) -> Result<bool>,
    G: Fn(&[Mod]) -> Result<&Mod>,
{
    let hashes: Vec<String> = if all {
        data::query_filtered(connection, None, None, None)?
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
        let matches = data::query_filtered(connection, Some(&hash), None, None)?;
        if matches.is_empty() {
            return Err(Error::Vault(VaultError::NotFound(hash)));
        }

        let value = resolve(&matches)?;
        if !yes && confirm(value)? {
            continue;
        }

        file::check_exists(&value.path)?;
        fs::remove_file(&value.path)?;
        file::remove_empty_dirs(&get_base_mod_dir_path()?)?;

        data::remove(connection, &value.hash)?;
    }

    Ok(())
}
