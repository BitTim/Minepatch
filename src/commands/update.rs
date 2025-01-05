/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       update.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:21
 */

use crate::common::error;
use crate::common::error::{CommonError, ErrorType};
use self_update::backends::github::Update;
use self_update::cargo_crate_version;

pub fn update() -> error::Result<()> {
    let status = Update::configure()
        .repo_owner("BitTim")
        .repo_name(env!("CARGO_PKG_NAME"))
        .bin_name("minepatch")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()
        .map_err(|error| {
            CommonError::Wrapper(Box::new(error))
                .builder()
                .context("Cause", "Updater")
                .build()
        })?
        .update()
        .map_err(|error| {
            CommonError::Wrapper(Box::new(error))
                .builder()
                .context("Cause", "Updater")
                .build()
        })?;

    println!("Update status: '{}'!", status.version());
    Ok(())
}
