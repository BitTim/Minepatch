/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       rename.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:15
 */
use crate::common::file::filename_from_path;
use crate::instance::error::InstanceError;
use crate::instance::func::common::registry::{check_instance, find_instance_mut};
use crate::prelude::*;

pub fn rename(name: &str, new_name: &Option<String>) -> Result<()> {
    // TODO: Rework for SQLite
    let mut instances = /*file::read_all()?*/ vec![];
    let instance = check_instance(&instances, name, true)?.unwrap().1;

    let actual_new_name = match new_name {
        Some(name) => name,
        None => &filename_from_path(&instance.path)?.to_string(),
    };

    if name == actual_new_name {
        return Err(Error::Instance(InstanceError::NameNotChanged));
    }

    check_instance(&instances, &actual_new_name, false)?;
    let instance = find_instance_mut(&mut instances, name)?;
    instance.set_name(actual_new_name);

    //file::write_all(instances)?;
    Ok(())
}
