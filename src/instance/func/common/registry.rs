/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:05
 */
use crate::instance::data::Instance;
use crate::instance::error::InstanceError;
use crate::prelude::*;

pub(crate) fn check_instance<'a>(
    instances: &'a [Instance],
    name: &str,
    should_exist: bool,
) -> Result<Option<(usize, &'a Instance)>> {
    match instances
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.name == name)
    {
        Some((index, instance)) if should_exist => Ok(Some((index, instance))),
        None if !should_exist => Ok(None),
        Some(_) => Err(Error::Instance(InstanceError::NameTaken(name.to_owned()))),
        None => Err(Error::Instance(InstanceError::NameNotFound(
            name.to_owned(),
        ))),
    }
}

pub(crate) fn find_instance_mut<'a>(
    instances: &'a mut [Instance],
    name: &str,
) -> Result<&'a mut Instance> {
    instances
        .iter_mut()
        .find(|instance| instance.get_name() == name)
        .ok_or(Error::Instance(InstanceError::NameNotFound(
            name.to_owned(),
        )))
}
