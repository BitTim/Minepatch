/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:40
 */
use crate::commands::instance::data::Instance;
use crate::commands::instance::error::InstanceError;
use crate::common::error;
use crate::common::error::ErrorType;

pub(crate) fn check_instance<'a>(
    instances: &'a [Instance],
    name: &str,
    should_exist: bool,
) -> error::Result<Option<(usize, &'a Instance)>> {
    match instances
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.name == name)
    {
        Some((index, instance)) if should_exist => Ok(Some((index, instance))),
        None if !should_exist => Ok(None),
        Some(_) => Err(InstanceError::NameTaken
            .builder()
            .context("Name", name)
            .build()),
        None => Err(InstanceError::NameNotFound
            .builder()
            .context("Name", name)
            .build()),
    }
}

pub(crate) fn find_instance_mut<'a>(
    instances: &'a mut [Instance],
    name: &str,
) -> error::Result<&'a mut Instance> {
    instances
        .iter_mut()
        .find(|instance| instance.get_name() == name)
        .ok_or(
            InstanceError::NameNotFound
                .builder()
                .context("Name", name)
                .build(),
        )
}
