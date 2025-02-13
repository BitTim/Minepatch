/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       func.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 23:55
 */
use crate::common::msg::Event;
use crate::prelude::*;
use std::sync::mpsc;
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

pub(crate) fn end_progress(
    tx: &Sender<Event>,
    process: Process,
    message: Option<Message>,
) -> Result<()> {
    Ok(tx.send(Event::ProgressFinish { process, message })?)
}

pub(crate) fn confirm(tx: &Sender<Event>, message: Message) -> Result<bool> {
    let (tx2, rx) = mpsc::channel();
    tx.send(Event::Confirm { tx: tx2, message })?;

    Ok(rx.recv()?)
}

pub(crate) fn select(
    tx: &Sender<Event>,
    options: Vec<String>,
    message: Message,
    multiselect: bool,
) -> Result<String> {
    let (tx2, rx) = mpsc::channel();

    tx.send(Event::Select {
        tx: tx2,
        options,
        multiselect,
        message,
    })?;

    Ok(rx.recv()?)
}
