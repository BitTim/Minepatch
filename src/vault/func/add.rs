/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       add.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 17:56
 */

use crate::common::file;
use crate::msg::Message;
use crate::prelude::*;
use crate::vault::data;
use crate::vault::data::Mod;
use crate::vault::func::common::meta::{detect_loader, extract_meta};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub fn add<F>(
    connection: &Connection,
    path: &Path,
    overwrite: bool,
    handle_warning: F,
) -> Result<String>
where
    F: FnOnce(Message),
{
    file::check_exists(path)?;
    let hash = file::hash_file(path)?;

    if data::exists(connection, &hash)? && !overwrite {
        handle_warning(
            Message::new("Mod is already registered in vault")
                .context("Path", &path.display().to_string())
                .context("Hash", &hash),
        );
        return Ok(hash);
    }

    let loader_result = detect_loader(path)?;
    if loader_result.is_none() {
        handle_warning(
            Message::new("No compatible loader detected for the provided file")
                .context("Path", &path.display().to_string()),
        )
    }

    let filename = file::filename_from_path(path)?;
    let (meta, mod_file_path) = extract_meta(loader_result, filename)?;

    fs::copy(path, &mod_file_path)?;
    data::insert(connection, Mod::new(&hash, &mod_file_path, meta))?;

    Ok(hash)
}
