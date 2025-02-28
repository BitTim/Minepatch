/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event;
use crate::common::event::Event;
use crate::db::Repo;
use crate::error::Error;
use crate::instance::data::{InstanceFilter, InstanceRepo};
use crate::instance::{InstanceError, InstanceMessage, InstanceProcess};
use crate::prelude::*;
use crate::{bundle, file, hash, patch};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, name: &str, exist_only: bool) -> Result<()> {
    event::init_progress(tx, Process::Instance(InstanceProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Instance(InstanceProcess::Validate),
        Message::Instance(InstanceMessage::ValidateStatus {
            name: name.to_owned(),
        }),
    )?;
    let query = InstanceFilter::ByExactName {
        name: name.to_owned(),
    };
    let instance = InstanceRepo::query_single(conn, &query)?;

    if exist_only {
        event::end_progress(tx, Process::Instance(InstanceProcess::Validate), None)?;
        return Ok(());
    }

    let mod_paths = file::mod_paths_from_instance_path(&instance.path)?;
    let src_dir_hash = hash::hash_state_from_path(tx, &mod_paths)?;
    let sim_hashes = patch::simulate(conn, tx, &instance.patch, &instance.bundle)?;
    let sim_dir_hash = hash::hash_state(&sim_hashes);

    if src_dir_hash != sim_dir_hash {
        return Err(Error::Instance(InstanceError::StateMismatch {
            present_hash: src_dir_hash,
            sim_hash: sim_dir_hash,
        }));
    }

    bundle::validate(conn, tx, &instance.bundle, false)?;
    patch::validate(conn, tx, &instance.patch, &instance.bundle, false)?;

    event::end_progress(tx, Process::Instance(InstanceProcess::Validate), None)?;
    Ok(())
}
