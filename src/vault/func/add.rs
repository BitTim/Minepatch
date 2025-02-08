/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 00:34
 */

use crate::common::{file, hash};
use crate::db::Repo;
use crate::prelude::*;
use crate::vault::data::{Mod, ModFilter, VaultRepo};
use crate::vault::func::common::meta::{detect_loader, extract_meta};
use crate::vault::VaultError;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub fn add<F>(
    connection: &Connection,
    path: &Path,
    overwrite: bool,
    handle_warning: &F,
) -> Result<String>
where
    F: Fn(Error),
{
    file::check_exists(path)?;
    let hash = hash::hash_file(path)?;

    let exists_query = ModFilter::QueryHashExact {
        hash: hash.to_owned(),
    };

    if VaultRepo::exists(connection, &exists_query)? && !overwrite {
        handle_warning(Error::Vault(VaultError::AlreadyExists {
            path: path.display().to_string(),
            hash: hash.to_owned(),
        }));
        return Ok(hash);
    }

    let loader_result = detect_loader(path)?;
    if loader_result.is_none() {
        handle_warning(Error::Vault(VaultError::NoLoaderDetected {
            path: path.display().to_string(),
        }))
    }

    let filename = file::filename_from_path(path)?;
    let (meta, mod_file_path) = extract_meta(loader_result, filename)?;

    fs::copy(path, &mod_file_path)?;
    VaultRepo::insert(connection, Mod::new(&hash, &mod_file_path, meta))?;

    Ok(hash)
}
