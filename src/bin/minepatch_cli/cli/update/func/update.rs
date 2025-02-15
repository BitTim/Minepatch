/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       update.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:41
 */
use minepatch::prelude::*;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use std::sync::mpsc::Sender;

pub fn update(tx: &Sender<Event>) -> Result<()> {
    let status = Update::configure()
        .repo_owner("BitTim")
        .repo_name(env!("CARGO_PKG_NAME"))
        .bin_name("minepatch")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    let output = format!("Update status: '{}'!", status.version());
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
