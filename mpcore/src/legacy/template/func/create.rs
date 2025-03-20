/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::Repo;
use crate::legacy::common::event;
use crate::legacy::template::data::Template;
use crate::legacy::template::data::{TemplateFilter, TemplateRepo};
use crate::legacy::template::{TemplateError, TemplateMessage, TemplateProcess};
use crate::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub fn create(
    conn: &Connection,
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
    if TemplateRepo::exists(conn, &exists_query)? {
        return Err(Error::Template(TemplateError::NameTaken(name.to_owned())));
    }

    let template = Template::new(name, loader, version, download);
    TemplateRepo::insert(conn, template.to_owned())?;

    event::end_progress(
        tx,
        Process::Template(TemplateProcess::Create),
        Some(Message::Template(TemplateMessage::CreateSuccess {
            template: Box::new(template),
        })),
    )?;
    Ok(())
}
