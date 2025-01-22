/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 03:40
 */
use crate::common::file;
use crate::pack::data::Pack;
use crate::pack::error::PackError;
use crate::pack::func::common::get_mod_paths;
use crate::pack::{exists, insert};
use crate::prelude::*;
use crate::template::TemplateError;
use crate::{instance, patch, template, vault};
use rusqlite::Connection;
use sha256::Sha256Digest;
use std::path::PathBuf;

const INIT_PATCH_NAME: &str = "init";

pub fn create(
    connection: &Connection,
    name: &str,
    description: Option<String>,
    template: Option<String>,
    from: Option<String>,
    instance: Option<String>,
) -> Result<()> {
    if exists(connection, name)? {
        return Err(Error::Pack(PackError::NameTaken(name.to_owned())));
    }

    if template.is_some() && !template::exists(connection, template.as_ref().unwrap())? {
        return Err(Error::Template(TemplateError::NotFound(name.to_owned())));
    }

    insert(connection, Pack::new(name, description, template))?;

    if let Some(from) = from {
        let mut path = PathBuf::from(from);
        path.push("mods");

        file::check_exists(&path)?;
        let mod_paths = get_mod_paths(&path)?;

        patch::create(connection, INIT_PATCH_NAME, "", &"".digest(), name)?;

        let mut hashes = vec![];
        for mod_path in mod_paths {
            let hash = vault::add(connection, &mod_path, false, |_| {})?;
            patch::include(connection, INIT_PATCH_NAME, name, &hash)?;
            hashes.push(hash);
        }

        if let Some(instance) = instance {
            let instance_name = match instance.is_empty() {
                true => None,
                false => Some(instance.to_owned()),
            };

            instance::func::link::link(&path, &instance_name)?;
        };
    }

    Ok(())
}
