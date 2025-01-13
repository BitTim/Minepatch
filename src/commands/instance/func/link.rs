/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:37
 */
use crate::commands::instance::common::registry::check_instance;
use crate::commands::instance::data::Instance;
use crate::common::error::ErrorType;
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::Output;
use crate::common::{error, file};
use std::fs;
use std::path::Path;

pub fn link(path: &Path, name: &Option<String>) -> error::Result<()> {
    if fs::exists(&path)? == false {
        return Err(FileError::PathNotFound
            .builder()
            .context("Path", &path.display().to_string())
            .build());
    }

    let actual_name = match name {
        Some(name) => name,
        None => filename_from_path(&path)?,
    };

    let mut instances = file::read_all()?;
    check_instance(&instances, actual_name, false)?;

    instances.push(Instance::new(actual_name, path));
    file::write_all(instances)?;

    StatusOutput::new(State::Success, "Linked instance")
        .context("Name", actual_name)
        .context("Path", &path.display().to_string())
        .print();
    Ok(())
}
