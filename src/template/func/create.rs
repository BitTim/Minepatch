/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:40
 */
use crate::common::msg;
use crate::db::Repo;
use crate::prelude::*;
use crate::template::data::{TemplateFilter, TemplateRepo};
use crate::template::{Template, TemplateError, TemplateMessage, TemplateProcess};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    loader: Option<String>,
    version: Option<String>,
    download: Option<String>,
) -> Result<()> {
    msg::init_progress(tx, Process::Template(TemplateProcess::Create), None)?;

    let exists_query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };
    if TemplateRepo::exists(connection, &exists_query)? {
        return Err(Error::Template(TemplateError::NameTaken(name.to_owned())));
    }

    TemplateRepo::insert(connection, Template::new(name, loader, version, download))?;

    msg::end_progress(
        tx,
        Process::Template(TemplateProcess::Create),
        Some(Message::Template(TemplateMessage::CreateSuccess {
            name: name.to_owned(),
        })),
    )?;
    Ok(())
}
