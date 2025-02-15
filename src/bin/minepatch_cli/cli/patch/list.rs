/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:38
 */
use crate::output::list_items::patch::PatchListItem;
use crate::output::table::TableOutput;
use minepatch::patch::query_multiple;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &Option<String>,
    pack: &Option<String>,
) -> Result<()> {
    let results = query_multiple(
        connection,
        name.to_owned().as_deref(),
        pack.to_owned().as_deref(),
    )?;
    let displays = results
        .iter()
        .map(|value| PatchListItem::from(connection, tx, value))
        .collect::<Result<Vec<PatchListItem>>>()?;

    let output = TableOutput::new(displays, "No patches present yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
