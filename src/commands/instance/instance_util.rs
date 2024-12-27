/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 18:13
 */
use crate::commands::instance::instance_error::InstanceError;
use crate::commands::instance::Instance;
use crate::common::error::ErrorType;
use std::error::Error;

pub fn check_instance<'a>(
    instances: &'a [Instance],
    name: &str,
    should_exist: bool,
) -> Result<Option<&'a Instance>, Box<dyn Error>> {
    match instances.iter().find(|instance| instance.name == name) {
        Some(instance) if should_exist => Ok(Some(instance)),
        None if !should_exist => Ok(None),
        Some(_) => Err(InstanceError::NameTaken
            .builder()
            .context(format!("Name: '{}'", name))
            .build()),
        None => Err(InstanceError::NameNotFound
            .builder()
            .context(format!("Name: '{}'", name))
            .build()),
    }
}

pub fn find_instance_mut<'a>(
    instances: &'a mut [Instance],
    name: &str,
) -> Result<&'a mut Instance, Box<dyn Error>> {
    instances
        .iter_mut()
        .find(|instance| instance.get_name() == name)
        .ok_or(
            InstanceError::NameNotFound
                .builder()
                .context(format!("Name: '{}'", name))
                .build(),
        )
}
