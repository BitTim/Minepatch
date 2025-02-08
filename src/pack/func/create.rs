/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 01:57
 */
use crate::common::file;
use crate::db::Repo;
use crate::pack::data::{Pack, PackFilter, PackRepo};
use crate::pack::error::PackError;
use crate::prelude::*;
use crate::{instance, patch, template, vault};
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::Path;

const INIT_PATCH_NAME: &str = "init";

pub fn create<F, G, H>(
    connection: &Connection,
    pack: Pack,
    from: Option<String>,
    instance: Option<String>,
    init_progress: &F,
    tick_progress: &G,
    handle_warning: &H,
) -> Result<()>
where
    F: Fn(u64),
    G: Fn(&str, &Path),
    H: Fn(Error),
{
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
        init_progress(mod_paths.len() as u64);

        let mut hashes = HashSet::new();
        for mod_path in mod_paths {
            let hash = vault::add(connection, &mod_path, false, handle_warning)?;
            tick_progress(&hash, &mod_path);
            hashes.insert(hash);
        }

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

            instance::link(
                connection,
                from.as_ref(),
                &instance_name,
                &name,
                INIT_PATCH_NAME,
            )?;
        };
    }

    Ok(())
}
