/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.01.25, 19:54
 */
use crate::common::file;
use crate::msg::Message;
use crate::pack::data;
use crate::pack::data::Pack;
use crate::pack::error::PackError;
use crate::prelude::*;
use crate::template::TemplateError;
use crate::{instance, patch, template, vault};
use rusqlite::Connection;
use sha256::Sha256Digest;
use std::path::Path;

const INIT_PATCH_NAME: &str = "init";

pub fn create<F, G, H>(
    connection: &Connection,
    pack: Pack,
    from: Option<String>,
    instance: Option<String>,
    init_progress: F,
    tick_progress: G,
    handle_warnings: H,
) -> Result<()>
where
    F: Fn(u64),
    G: Fn(&str, &Path),
    H: FnOnce(Vec<Message>),
{
    let name = pack.name.to_owned();
    let template = pack.template.to_owned();

    if data::exists(connection, &name)? {
        return Err(Error::Pack(PackError::NameTaken(name)));
    }

    if template.is_some() && !template::data::exists(connection, template.as_ref().unwrap())? {
        return Err(Error::Template(TemplateError::NotFound(name)));
    }

    data::insert(connection, pack)?;

    if let Some(from) = from {
        file::check_exists(from.as_ref())?;
        let mod_paths = file::mod_paths_from_instance_path(from.as_ref())?;
        init_progress(mod_paths.len() as u64);

        patch::create(connection, INIT_PATCH_NAME, &name, "", &"".digest())?;

        let mut warnings = vec![];
        let mut hashes = vec![];
        for mod_path in mod_paths {
            let hash = vault::add(connection, &mod_path, false, |warning| {
                warnings.push(warning)
            })?;
            patch::include(connection, INIT_PATCH_NAME, &name, &hash)?;

            tick_progress(&hash, &mod_path);
            hashes.push(hash);
        }

        handle_warnings(warnings);

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
