/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 10:17
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
    let results = query(
        connection,
        name.to_owned().as_deref(),
        pack.to_owned().as_deref(),
    )?;
    let displays = results
        .iter()
        .map(|value| PatchListItem::from(connection, value))
        .collect::<Result<Vec<PatchListItem>>>();

    TableOutput::new(displays?).print();
    Ok(())
}
