/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 18:58
 */
use crate::common::event::Event;
use crate::common::file;
use crate::db::Repo;
use crate::msg::Message;
use crate::pack::data::{Pack, PackFilter, PackRepo};
use crate::pack::error::PackError;
use crate::prelude::*;
use crate::{instance, patch, template, vault};
use colored::Colorize;
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::mpsc::Sender;
use uuid::Uuid;

const INIT_PATCH_NAME: &str = "init";

pub fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    pack: Pack,
    from: Option<String>,
    instance: Option<String>,
) -> Result<()> {
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
        let hash_prog_id = Uuid::new_v4();

        file::check_exists(from.as_ref())?;
        let mod_paths = file::mod_paths_from_instance_path(from.as_ref())?;
        tx.send(Event::Progress {
            id: hash_prog_id.to_owned(),
            title: "Hashing mod files".to_owned(),
            total: Some(mod_paths.len() as u64),
        })?;

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(connection, tx, &mod_path, false)?;
            hashes.insert(hash);
            tx.send(Event::ProgressTick {
                id: hash_prog_id.to_owned(),
                message: Message::new(
                    &format!("Mod file path: '{}'", mod_path.display().to_string().cyan())
                        .to_string(),
                ),
            })?;
        }

        tx.send(Event::ProgressFinish {
            id: hash_prog_id.to_owned(),
        })?;

        patch::create(
            connection,
            INIT_PATCH_NAME,
            &name,
            "",
            &hashes,
            &HashSet::new(),
        )?;

        if let Some(instance) = instance {
            let instance_name = match instance.is_empty() {
                true => None,
                false => Some(instance.to_owned()),
            };

            let spinner_id = Uuid::new_v4();
            tx.send(Event::Progress {
                id: spinner_id.to_owned(),
                title: "Linking instance".to_string(),
                total: None,
            })?;

            // FIXME: This will fail because of src_dir_hash
            instance::link(connection, from.as_ref(), &instance_name, &name)?;
            tx.send(Event::ProgressFinish {
                id: spinner_id.to_owned(),
            })?;
        };
    }

    Ok(())
}
