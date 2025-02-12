/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       update.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:44
 */

use minepatch::prelude::*;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;

pub fn update() -> Result<()> {
    let status = Update::configure()
        .repo_owner("BitTim")
        .repo_name(env!("CARGO_PKG_NAME"))
        .bin_name("minepatch")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: '{}'!", status.version());
    Ok(())
}
