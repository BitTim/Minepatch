/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.02.25, 23:28
 */
use crate::common::Repo;
use crate::file;
use crate::prelude::*;
use crate::vault::data::{Mod, VaultQueries, VaultRepo};
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
        let query_all = VaultQueries::QueryAll;
        VaultRepo::query_multiple(connection, &query_all)?
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
        let query = VaultQueries::QueryHashExact {
            hash: hash.to_owned(),
        };
        let matches = VaultRepo::query_multiple(connection, &query)?;
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

        let remove_query = VaultQueries::QueryHashExact {
            hash: value.hash.to_owned(),
        };
        VaultRepo::remove(connection, &remove_query)?;
    }

    Ok(())
}
