/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 18:31
 */
use crate::commands::instance::instance_error::InstanceError;
use crate::commands::instance::Instance;
use crate::common::error;
use crate::common::error::ErrorType;
use colored::Colorize;
use inquire::Confirm;

pub fn check_instance<'a>(
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
            .context(&format!("Name: '{}'", name))
            .build()),
        None => Err(InstanceError::NameNotFound
            .builder()
            .context(&format!("Name: '{}'", name))
            .build()),
    }
}

pub fn find_instance_mut<'a>(
    instances: &'a mut [Instance],
    name: &str,
) -> error::Result<&'a mut Instance> {
    instances
        .iter_mut()
        .find(|instance| instance.get_name() == name)
        .ok_or(
            InstanceError::NameNotFound
                .builder()
                .context(&format!("Name: '{}'", name))
                .build(),
        )
}

pub fn confirm_unlink(instance: &Instance) -> error::Result<bool> {
    let ans = Confirm::new(&format!(
        "Are you sure you want to unlink '{}'?",
        instance.name
    ))
    .with_default(false)
    .with_help_message(&format!("Path: '{}'", &instance.path.display().to_string()))
    .prompt()?;

    if !ans {
        println!(
            "{}Did not unlink instance\n\t{}",
            "cancelled: ".yellow().bold(),
            format!("Name: '{}'", instance.name).green(),
        );
    }

    Ok(ans)
}
