/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 21:30
 */
use crate::prelude::*;
use crate::{file, instance, patch, vault};
use rusqlite::Connection;
use std::{fs, process};

pub fn apply(connection: &Connection, instance: &str, patch: &str) -> Result<()> {
    let instance = instance::query_single(connection, instance)?;

    let mods_path = instance.path.join("mods");
    let tmp_mods_path = mods_path.with_extension(process::id().to_string());

    let cleanup = || -> Result<()> {
        fs::remove_dir_all(&mods_path)?;
        fs::rename(&tmp_mods_path, &mods_path)?;
        Ok(())
    };

    fs::rename(&mods_path, &tmp_mods_path)?;
    fs::create_dir(&mods_path)?;

    let hashes = patch::simulate(connection, patch, &instance.pack)?;

    for hash in hashes {
        let mod_entry = vault::query_single(connection, &hash)?;
        let filename = file::filename_from_path(&mod_entry.path)?;
        let path = mods_path.join(filename);

        if let Err(err) = symlink::symlink_file(&mod_entry.path, path) {
            cleanup()?;
            return Err(Error::from(err));
        }
    }

    if let Err(err) = instance::update_patch(connection, &instance.name, patch) {
        cleanup()?;
        return Err(err);
    }

    let a = instance::query_single(connection, &instance.name)?;
    println!("{:?}", a);

    Ok(fs::remove_dir_all(&tmp_mods_path)?)
}
