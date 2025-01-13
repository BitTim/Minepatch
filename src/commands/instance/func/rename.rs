/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       rename.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:36
 */
use crate::commands::instance::common::registry::{check_instance, find_instance_mut};
use crate::commands::instance::error::InstanceError;
use crate::common::error::ErrorType;
use crate::common::file::filename_from_path;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::Output;
use crate::common::{error, file};

pub fn rename(name: &str, new_name: &Option<String>) -> error::Result<()> {
    let mut instances = file::read_all()?;
    let instance = check_instance(&instances, name, true)?.unwrap().1;

    let actual_new_name = match new_name {
        Some(name) => name,
        None => &filename_from_path(&instance.path)?.to_string(),
    };

    if name == actual_new_name {
        return Err(InstanceError::NameNotChanged
            .builder()
            .context("Name", name)
            .context("New Name", actual_new_name)
            .build());
    }

    check_instance(&instances, &actual_new_name, false)?;
    let instance = find_instance_mut(&mut instances, name)?;
    instance.set_name(actual_new_name);

    file::write_all(instances)?;

    StatusOutput::new(State::Success, "Renamed instance")
        .context("Old name", name)
        .context("New name", actual_new_name)
        .print();
    Ok(())
}
