/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 16:13
 */
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use minepatch::msg::Message;
use minepatch::pack;
use minepatch::pack::Pack;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::time::Duration;

pub(crate) fn create(
    connection: &Connection,
    name: &str,
    description: &Option<String>,
    template: &Option<String>,
    from: &Option<String>,
    instance: &Option<String>,
) -> Result<()> {
    let progress_bar = ProgressBar::hidden();
    progress_bar.set_style(ProgressStyle::with_template("{msg}\n{spinner} {wide_bar} {percent:>3} % ({human_pos:>5} / {human_len:5})\nElapsed: {elapsed_precise}\tETA: {eta_precise}")?);
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    pack::create(
        connection,
        Pack::new(name, description.to_owned(), template.to_owned()),
        from.to_owned(),
        instance.to_owned(),
        |size| {
            progress_bar.set_draw_target(ProgressDrawTarget::stderr());
            progress_bar.set_length(size);
        },
        |hash, path| {
            progress_bar.set_message(format!("File\t'{}'\nHash\t'{}'", path.display(), hash));
            progress_bar.inc(1);
        },
        |warnings| {
            for warning in warnings {
                progress_bar.suspend(|| StatusOutput::new(Status::Warning, warning).print());
            }
        },
    )?;

    progress_bar.finish_and_clear();

    StatusOutput::new(
        Status::Success,
        Message::new("Created new pack").context("Name", name),
    )
    .print();
    Ok(())
}
