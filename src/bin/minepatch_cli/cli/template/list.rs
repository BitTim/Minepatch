/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::output::list_items::template::TemplateListItem;
use crate::output::table::TableOutput;
use minepatch::prelude::*;
use minepatch::template;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(conn: &Connection, tx: &Sender<Event>, name: &Option<String>) -> Result<()> {
    let templates = template::query_multiple(conn, name.to_owned().as_deref())?
        .iter()
        .map(TemplateListItem::from)
        .collect::<Vec<TemplateListItem>>();

    let output = TableOutput::new(templates, "No templates added yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
