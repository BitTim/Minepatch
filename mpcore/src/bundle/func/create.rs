/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 07:32
 */
use crate::bundle::data::{Bundle, BundleRepo};
use crate::bundle::error::BundleError;
use crate::bundle::msg::BundleMessage;
use crate::bundle::BundleProcess;
use crate::common::event::Event;
use crate::common::{event, file};
use crate::db::Repo;
use crate::prelude::*;
use crate::{instance, patch, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::Sender;

const INIT_PATCH_NAME: &str = "init";

pub fn create(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    description: Option<&str>,
    from: Option<&Path>,
    instance: Option<&str>,
) -> Result<()> {
    event::init_progress(tx, Process::Bundle(BundleProcess::Create), None)?;

    if BundleRepo::exists(conn, name)? {
        return Err(Error::Bundle(BundleError::NameTaken(name.to_owned())));
    }

    let bundle = Bundle::new(name, description);
    BundleRepo::insert(conn, bundle.to_owned())?;

    if let Some(from) = from {
        file::check_exists(from)?;
        let mod_paths = file::mod_paths_from_instance_path(from)?;
        event::init_progress(
            tx,
            Process::Bundle(BundleProcess::AddModFiles),
            Some(mod_paths.len() as u64),
        )?;

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(conn, tx, &mod_path, false)?;
            hashes.insert(hash.clone());
            event::tick_progress(
                tx,
                Process::Bundle(BundleProcess::AddModFiles),
                Message::Bundle(BundleMessage::AddModFileStatus {
                    path: mod_path.to_path_buf(),
                    hash,
                }),
                1,
            )?;
        }

        event::end_progress(tx, Process::Bundle(BundleProcess::AddModFiles), None)?;
        patch::create(
            conn,
            tx,
            INIT_PATCH_NAME,
            name,
            "",
            &hashes,
            &HashSet::new(),
        )?;

        if let Some(instance) = instance {
            let instance_name = match instance.is_empty() {
                true => None,
                false => Some(instance),
            };

            instance::link(conn, tx, from, instance_name, Some(name))?;
        };
    }

    event::end_progress(
        tx,
        Process::Bundle(BundleProcess::Create),
        Some(Message::Bundle(BundleMessage::CreateSuccess {
            bundle: Box::new(bundle),
        })),
    )?;
    Ok(())
}
