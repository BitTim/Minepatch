/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       confirm.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 12:58
 */
use crate::instance::data::Instance;
use crate::prelude::*;
use inquire::Confirm;

pub(crate) fn confirm_unlink(instance: &Instance) -> Result<bool> {
    let ans = Confirm::new(&format!(
        "Are you sure you want to unlink '{}'?",
        instance.name
    ))
    .with_default(false)
    .with_help_message(&format!("Path: '{}'", &instance.path.display().to_string()))
    .prompt()?;

    Ok(ans)
}
