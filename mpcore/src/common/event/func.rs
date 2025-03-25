/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       func.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 18:26
 */
use crate::common::event::{Event, EventError};
use crate::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub(crate) fn init_progress(
    tx: &Sender<Event>,
    process: Process,
    total: Option<u64>,
) -> Result<()> {
    Ok(tx.send(Event::Progress { process, total })?)
}

pub(crate) fn tick_progress(
    tx: &Sender<Event>,
    process: Process,
    message: Message,
    increment: u64,
) -> Result<()> {
    Ok(tx.send(Event::ProgressTick {
        process,
        message,
        increment,
    })?)
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

pub(crate) fn select<T, F>(
    tx: &Sender<Event>,
    options: HashSet<T>,
    message: Message,
    msg_generator: F,
) -> Result<T>
where
    T: Clone + Hash + Eq + PartialEq,
    F: Fn(T) -> Message,
{
    if options.is_empty() {
        return Err(Error::Event(EventError::NoOptions));
    }

    if options.len() < 2 {
        return options
            .into_iter()
            .next()
            .ok_or(Error::Event(EventError::InvalidSelection));
    }

    let options = options.into_iter().collect::<Vec<T>>();
    let option_messages = options
        .clone()
        .into_iter()
        .map(msg_generator)
        .collect::<Vec<Message>>();

    let (tx2, rx) = mpsc::channel();
    tx.send(Event::Select {
        tx: tx2,
        options: option_messages,
        message,
    })?;

    let response = rx.recv()?;
    options
        .get(response)
        .map(ToOwned::to_owned)
        .ok_or(Error::Event(EventError::InvalidSelection))
}

pub(crate) fn warning(tx: &Sender<Event>, error: Box<Error>) -> Result<()> {
    Ok(tx.send(Event::Warning { warning: error })?)
}
