/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:18
 */
use crate::commands::instance::instance_error::InstanceError;
use crate::commands::instance::{instance_util, Instance, InstanceDisplay};
use crate::common::error::ErrorType;
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::output::status::{State, StatusOutput};
use crate::common::output::table::TableOutput;
use crate::common::output::Output;
use crate::common::{error, file};
use std::fs;
use std::path::Path;
use tabled::settings::object::Columns;

pub fn list() -> error::Result<()> {
    let instances: Vec<Instance> = file::read_all()?;
    let instance_displays = instances
        .iter()
        .map(|instance| instance.to_display())
        .collect::<Vec<InstanceDisplay>>();

    TableOutput::new(instance_displays)
        .center(Columns::new(2..3))
        .print();
    Ok(())
}

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
    instance_util::check_instance(&instances, actual_name, false)?;

    instances.push(Instance::new(actual_name, path));
    file::write_all(instances)?;

    StatusOutput::new(State::Success, "Linked instance")
        .context("Name", actual_name)
        .context("Path", &path.display().to_string())
        .print();
    Ok(())
}

pub fn rename(name: &str, new_name: &Option<String>) -> error::Result<()> {
    let mut instances = file::read_all()?;
    let instance = instance_util::check_instance(&instances, name, true)?
        .unwrap()
        .1;

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

    instance_util::check_instance(&instances, &actual_new_name, false)?;
    let instance = instance_util::find_instance_mut(&mut instances, name)?;
    instance.set_name(actual_new_name);

    file::write_all(instances)?;

    StatusOutput::new(State::Success, "Renamed instance")
        .context("Old name", name)
        .context("New name", actual_new_name)
        .print();
    Ok(())
}

pub fn unlink(name: &Option<String>, all: &bool, yes: &bool) -> error::Result<()> {
    let mut instances = file::read_all()?;

    let names: Vec<String> = if *all {
        instances
            .iter()
            .map(|instance: &Instance| instance.name.to_string())
            .collect()
    } else {
        match name {
            Some(name) => vec![name.to_string()],
            None => return Err(InstanceError::NameNotFound.builder().build()),
        }
    };

    for name in names {
        let (index, instance) = instance_util::check_instance(&instances, &name, true)?.unwrap();
        if !yes && !instance_util::confirm_unlink(instance)? {
            StatusOutput::new(State::Abort, "Did not unlink instance")
                .context("Name", &instance.name)
                .print();
            continue;
        }
        instances.remove(index);

        StatusOutput::new(State::Success, "Unlinked instance")
            .context("Name", &*name)
            .print();
    }

    file::write_all(instances)?;
    Ok(())
}
