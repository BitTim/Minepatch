/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       link.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */
use crate::instance::data::Instance;
use crate::instance::func::common::registry::check_instance;
use crate::util::error::ErrorType;
use crate::util::file::error::FileError;
use crate::util::file::filename_from_path;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::util::{error, file};
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
