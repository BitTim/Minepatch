/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       update.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:43
 */

use crate::util::error;
use crate::util::error::{CommonError, ErrorType};
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
