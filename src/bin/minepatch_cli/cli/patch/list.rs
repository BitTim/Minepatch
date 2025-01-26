/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 03:01
 */
use crate::output::list_items::patch::PatchListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::patch::query;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn list(
    connection: &Connection,
    name: &Option<String>,
    pack: &Option<String>,
) -> Result<()> {
    let results = query(connection, name.to_owned(), pack.to_owned())?;
    let displays = results
        .iter()
        .map(|value| PatchListItem::from(connection, value))
        .collect::<Result<Vec<PatchListItem>>>();

    TableOutput::new(displays?).print();
    Ok(())
}
