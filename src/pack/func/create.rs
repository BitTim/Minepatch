/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:54
 */
use crate::common::msg::Event;
use crate::common::{file, msg};
use crate::db::Repo;
use crate::pack::data::{Pack, PackFilter, PackRepo};
use crate::pack::error::PackError;
use crate::pack::msg::{PackContext, PackMessage, PackProcess};
use crate::prelude::*;
use crate::{instance, patch, template, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

const INIT_PATCH_NAME: &str = "init";

pub fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    pack: Pack,
    from: Option<&str>,
    instance: Option<&str>,
) -> Result<()> {
    msg::init_progress(tx, Process::Pack(PackProcess::Create), None)?;

    let name = pack.name.to_owned();
    let template = pack.template.to_owned();
    let exists_query = PackFilter::QueryExactName {
        name: name.to_owned(),
    };

    if PackRepo::exists(connection, &exists_query)? {
        return Err(Error::Pack(PackError::NameTaken(name)));
    }

    if template.is_some() {
        template::validate(connection, template.as_ref().unwrap())?;
    }

    PackRepo::insert(connection, pack)?;

    if let Some(from) = from {
        file::check_exists(from.as_ref())?;
        let mod_paths = file::mod_paths_from_instance_path(from.as_ref())?;
        msg::init_progress(
            tx,
            Process::Pack(PackProcess::AddModFiles),
            Some(mod_paths.len() as u64),
        )?;

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(connection, tx, &mod_path, false)?;
            hashes.insert(hash.clone());
            msg::tick_progress(
                tx,
                Process::Pack(PackProcess::AddModFiles),
                Message::Pack(PackMessage::AddModFileStatus(vec![
                    PackContext::Path(mod_path.to_path_buf()),
                    PackContext::Hash(hash),
                ])),
            )?;
        }

        msg::end_progress(tx, Process::Pack(PackProcess::AddModFiles))?;
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

    msg::end_progress(tx, Process::Pack(PackProcess::Create))?;
    Ok(())
}
