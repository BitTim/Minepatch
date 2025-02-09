/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 22:15
 */
use crate::msg::Message;
use crate::prelude::*;
use crate::progress::event::Event;
use std::sync::mpsc::Sender;
use uuid::Uuid;

pub fn init_progress(tx: &Sender<Event>, title: &str, total: Option<u64>) -> Result<Uuid> {
    let id = Uuid::new_v4();
    tx.send(Event::Progress {
        id: id.to_owned(),
        title: title.to_owned(),
        total,
    })?;

    Ok(id)
}

pub fn tick_progress(tx: &Sender<Event>, id: &Uuid, message: Message) -> Result<()> {
    Ok(tx.send(Event::ProgressTick {
        id: id.to_owned(),
        message,
    })?)
}

pub fn end_progress(tx: &Sender<Event>, id: Uuid) -> Result<()> {
    Ok(tx.send(Event::ProgressFinish { id })?)
}
