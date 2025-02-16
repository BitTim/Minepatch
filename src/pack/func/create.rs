/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */
use crate::common::event::Event;
use crate::common::{event, file};
use crate::db::Repo;
use crate::pack::data::{Pack, PackFilter, PackRepo};
use crate::pack::error::PackError;
use crate::pack::msg::{PackMessage, PackProcess};
use crate::prelude::*;
use crate::{instance, patch, template, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

const INIT_PATCH_NAME: &str = "init";

pub fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    description: Option<&str>,
    template: Option<&str>,
    from: Option<&str>,
    instance: Option<&str>,
) -> Result<()> {
    event::init_progress(tx, Process::Pack(PackProcess::Create), None)?;
    let exists_query = PackFilter::QueryExactName {
        name: name.to_owned(),
    };

    if PackRepo::exists(connection, &exists_query)? {
        return Err(Error::Pack(PackError::NameTaken(name.to_owned())));
    }

    if template.is_some() {
        template::validate(connection, tx, template.as_ref().unwrap())?;
    }

    let pack = Pack::new(name, description, template);
    PackRepo::insert(connection, pack.to_owned())?;

    if let Some(from) = from {
        file::check_exists(from.as_ref())?;
        let mod_paths = file::mod_paths_from_instance_path(from.as_ref())?;
        event::init_progress(
            tx,
            Process::Pack(PackProcess::AddModFiles),
            Some(mod_paths.len() as u64),
        )?;

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(connection, tx, &mod_path, false)?;
            hashes.insert(hash.clone());
            event::tick_progress(
                tx,
                Process::Pack(PackProcess::AddModFiles),
                Message::Pack(PackMessage::AddModFileStatus {
                    path: mod_path.to_path_buf(),
                    hash,
                }),
            )?;
        }

        event::end_progress(tx, Process::Pack(PackProcess::AddModFiles), None)?;
        patch::create(
            connection,
            tx,
            INIT_PATCH_NAME,
            &name,
            "",
            &hashes,
            &HashSet::new(),
        )?;

        if let Some(instance) = instance {
            let instance_name = match instance.is_empty() {
                true => None,
                false => Some(instance),
            };

            instance::link(connection, tx, from.as_ref(), instance_name, Some(&name))?;
        };
    }

    event::end_progress(
        tx,
        Process::Pack(PackProcess::Create),
        Some(Message::Pack(PackMessage::CreateSuccess {
            pack: Box::new(pack),
        })),
    )?;
    Ok(())
}
