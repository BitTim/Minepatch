/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       confirm.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:24
 */
use crate::instance::data::Instance;
use crate::util::error;
use inquire::Confirm;

pub(crate) fn confirm_unlink(instance: &Instance) -> error::Result<bool> {
    let ans = Confirm::new(&format!(
        "Are you sure you want to unlink '{}'?",
        instance.name
    ))
    .with_default(false)
    .with_help_message(&format!("Path: '{}'", &instance.path.display().to_string()))
    .prompt()?;

    Ok(ans)
}
