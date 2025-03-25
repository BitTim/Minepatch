/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       apply.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 18:04
 */
use crate::common::event;
use crate::common::event::Event;
use crate::instance::data::InstanceRepo;
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::prelude::*;
use crate::vault::VaultRepo;
use crate::{file, instance, patch};
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use std::{fs, process};

pub fn apply(conn: &Connection, tx: &Sender<Event>, instance: &str, patch: &str) -> Result<()> {
    event::init_progress(tx, Process::Instance(InstanceProcess::Apply), None)?;
    let instance_options = InstanceRepo::by_name_many(conn, Some(instance), false)?;
    let instance = event::select(
        tx,
        instance_options,
        Message::Instance(InstanceMessage::Select),
        |instance| Message::Instance(InstanceMessage::Option { instance }),
    )?;

    let mods_path = instance.path.join("mods");
    let tmp_mods_path = mods_path.with_extension(process::id().to_string());

    let cleanup = || -> Result<()> {
        fs::remove_dir_all(&mods_path)?;
        fs::rename(&tmp_mods_path, &mods_path)?;
        Ok(())
    };

    fs::rename(&mods_path, &tmp_mods_path)?;
    fs::create_dir(&mods_path)?;

    let hashes = patch::simulate(conn, tx, patch, &instance.bundle)?;

    for hash in hashes {
        let mod_entry = VaultRepo::by_hash(conn, &hash)?;
        let filename = file::filename_from_path(&mod_entry.path)?;
        let path = mods_path.join(filename);

        if let Err(err) = symlink::symlink_file(&mod_entry.path, path) {
            cleanup()?;
            return Err(Error::from(err));
        }
    }

    if let Err(err) = instance::update_patch(conn, &instance.name, patch) {
        cleanup()?;
        return Err(err);
    }

    fs::remove_dir_all(&tmp_mods_path)?;
    event::end_progress(
        tx,
        Process::Instance(InstanceProcess::Apply),
        Some(Message::Instance(InstanceMessage::ApplySuccess {
            bundle: instance.bundle,
            patch: patch.to_owned(),
        })),
    )?;

    Ok(())
}
