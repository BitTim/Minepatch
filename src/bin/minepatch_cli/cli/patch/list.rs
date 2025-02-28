/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.02.25, 00:40
 */
use crate::output::list_items::patch::PatchListItem;
use crate::output::table::TableOutput;
use minepatch::patch::query_multiple;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &Option<String>,
    bundle: &Option<String>,
) -> Result<()> {
    let results = query_multiple(
        conn,
        name.to_owned().as_deref(),
        bundle.to_owned().as_deref(),
    )?;
    let displays = results
        .iter()
        .map(|value| PatchListItem::from(conn, tx, value))
        .collect::<Result<Vec<PatchListItem>>>()?;

    let output = TableOutput::new(displays, "No patches present yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
