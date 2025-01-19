/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:13
 */
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::instance::data::Instance;
use crate::instance::func::common::registry::check_instance;
use crate::prelude::*;
use std::fs;
use std::path::Path;

pub fn link(path: &Path, name: &Option<String>) -> Result<()> {
    if fs::exists(&path)? == false {
        return Err(Error::File(FileError::PathNotFound(
            path.display().to_string(),
        )));
    }

    let actual_name = match name {
        Some(name) => name,
        None => filename_from_path(&path)?,
    };

    // TODO: Rework for SQLite
    let mut instances = /*file::read_all()?*/ vec![];
    check_instance(&instances, actual_name, false)?;

    instances.push(Instance::new(actual_name, path));
    //file::write_all(instances)?;

    Ok(())
}
