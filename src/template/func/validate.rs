/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       validate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:41
 */
use crate::common::msg;
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::{TemplateError, TemplateMessage, TemplateProcess};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn validate(connection: &Connection, tx: &Sender<Event>, name: &str) -> Result<()> {
    msg::init_progress(tx, Process::Template(TemplateProcess::Validate), None)?;

    let query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };

    if !TemplateRepo::exists(connection, &query)? {
        return Err(Error::Template(TemplateError::NotFound(name.to_owned())));
    }

    msg::end_progress(
        tx,
        Process::Template(TemplateProcess::Validate),
        Some(Message::Template(TemplateMessage::ValidateSuccess {
            name: name.to_owned(),
        })),
    )?;
    Ok(())
}
