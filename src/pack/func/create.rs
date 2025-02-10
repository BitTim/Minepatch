/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:51
 */
use crate::common::file;
use crate::common::progress::event::Event;
use crate::db::Repo;
use crate::msg::Message;
use crate::pack::data::{Pack, PackFilter, PackRepo};
use crate::pack::error::PackError;
use crate::prelude::*;
use crate::{instance, patch, progress, template, vault};
use colored::Colorize;
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
    let spinner_id = progress::init_progress(tx, "Creating pack", None)?;

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
        let hash_prog_id =
            progress::init_progress(tx, "Hashing mod files", Some(mod_paths.len() as u64))?;

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(connection, tx, &mod_path, false)?;
            hashes.insert(hash);
            progress::tick_progress(
                tx,
                &hash_prog_id,
                Message::new(
                    &format!("Mod file path: '{}'", mod_path.display().to_string().cyan())
                        .to_string(),
                ),
            )?;
        }

        progress::end_progress(tx, hash_prog_id)?;
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

    progress::end_progress(tx, spinner_id)?;
    Ok(())
}
