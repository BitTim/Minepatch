/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:37
 */
use crate::output::list_items::pack::PackListItem;
use crate::output::table::TableOutput;
use minepatch::pack;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &Option<String>,
) -> Result<()> {
    let results = pack::query(connection, name.to_owned().as_deref())?;
    let list_items = results
        .iter()
        .map(|value| PackListItem::from(connection, tx, value))
        .collect::<Result<Vec<PackListItem>>>();

    let output = TableOutput::new(list_items?, "No mod packs added yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
