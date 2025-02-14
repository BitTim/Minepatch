/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:11
 */
use crate::common::event;
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
    event::init_progress(tx, Process::Template(TemplateProcess::Create), None)?;

    let exists_query = TemplateFilter::QueryNameExact {
        name: name.to_owned(),
    };
    if TemplateRepo::exists(connection, &exists_query)? {
        return Err(Error::Template(TemplateError::NameTaken(name.to_owned())));
    }

    let template = Template::new(name, loader, version, download);
    TemplateRepo::insert(connection, template.to_owned())?;

    event::end_progress(
        tx,
        Process::Template(TemplateProcess::Create),
        Some(Message::Template(TemplateMessage::CreateSuccess {
            template: Box::new(template),
        })),
    )?;
    Ok(())
}
