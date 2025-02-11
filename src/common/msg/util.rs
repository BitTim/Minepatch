/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:00
 */
use crate::common::msg::Event;
use crate::prelude::*;
use std::sync::mpsc::Sender;

pub(crate) fn init_progress(
    tx: &Sender<Event>,
    process: Process,
    total: Option<u64>,
) -> Result<()> {
    Ok(tx.send(Event::Progress { process, total })?)
}

pub(crate) fn tick_progress(tx: &Sender<Event>, process: Process, message: Message) -> Result<()> {
    Ok(tx.send(Event::ProgressTick { process, message })?)
}

pub(crate) fn end_progress(tx: &Sender<Event>, process: Process) -> Result<()> {
    Ok(tx.send(Event::ProgressFinish { process })?)
}
