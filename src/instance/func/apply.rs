/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:11
 */
use crate::common::msg;
use crate::common::msg::Event;
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::prelude::*;
use crate::{file, instance, patch, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use std::{fs, process};

pub fn apply(
    connection: &Connection,
    tx: &Sender<Event>,
    instance: &str,
    patch: &str,
) -> Result<()> {
    msg::init_progress(tx, Process::Instance(InstanceProcess::Apply), None)?;
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

    let hashes = patch::simulate(connection, tx, patch, &instance.pack)?;

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

    fs::remove_dir_all(&tmp_mods_path)?;
    msg::end_progress(
        tx,
        Process::Instance(InstanceProcess::Apply),
        Some(Message::Instance(InstanceMessage::ApplySuccess {
            pack: instance.pack,
            patch: patch.to_owned(),
        })),
    )?;

    Ok(())
}
