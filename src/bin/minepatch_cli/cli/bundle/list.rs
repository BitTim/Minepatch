/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 05:47
 */
use crate::output::list_items::bundle::PackListItem;
use crate::output::table::TableOutput;
use minepatch::bundle;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(conn: &Connection, tx: &Sender<Event>, name: &Option<String>) -> Result<()> {
    let results = bundle::query_multiple(conn, name.to_owned().as_deref())?;
    let list_items = results
        .iter()
        .map(|value| PackListItem::from(conn, tx, value))
        .collect::<Result<Vec<PackListItem>>>();

    let output = TableOutput::new(list_items?, "No mod packs added yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
