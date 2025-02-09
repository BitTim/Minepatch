/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       detect.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 00:20
 */
use crate::patch;
use crate::prelude::*;
use rusqlite::Connection;

// TODO: Use simulation and this detect method in order to find applied patch during linking. Src_Dir_Hash will be retired
pub fn detect(connection: &Connection, instance: &str, pack: Option<&str>) -> Result<()> {
    let patches = patch::query_multiple(connection, None, pack)?;

    patches
        .iter()
        .map(|patch| patch::simulate_dir_hash(connection, &patch.name, &patch.pack));

    Ok(())
}
