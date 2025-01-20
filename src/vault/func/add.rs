/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 02:38
 */

use crate::common::file;
use crate::prelude::*;
use crate::vault::data::repository::{exists, insert};
use crate::vault::data::Mod;
use crate::vault::error::VaultError;
use crate::vault::func::common::meta::{detect_loader, extract_meta};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub fn add(connection: &Connection, path: &Path, overwrite: bool) -> Result<String> {
    file::check_exists(path)?;
    let hash = file::hash_file(path)?;

    if exists(connection, &hash)? && !overwrite {
        return Err(Error::Vault(VaultError::HashExists(hash)));
    }

    let loader_result = detect_loader(path)?;
    let filename = file::filename_from_path(path)?;
    let (meta, mod_file_path) = extract_meta(loader_result, filename)?;

    fs::copy(path, &mod_file_path)?;
    insert(connection, Mod::new(&hash, &mod_file_path, meta))?;

    Ok(hash)
}
