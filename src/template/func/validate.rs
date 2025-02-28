/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::common::event;
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::{TemplateError, TemplateMessage, TemplateProcess};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(conn: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    event::init_progress(tx, Process::Template(TemplateProcess::Validate), None)?;
    event::tick_progress(
        tx,
        Process::Template(TemplateProcess::Validate),
        Message::Template(TemplateMessage::ValidateStatus {
            name: name.to_owned(),
        }),
    )?;

    let query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };

    if !TemplateRepo::exists(conn, &query)? {
        return Err(Error::Template(TemplateError::NotFound(name.to_owned())));
    }

    event::end_progress(tx, Process::Template(TemplateProcess::Validate), None)?;
    Ok(())
}
