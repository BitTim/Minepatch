/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 22:16
 */
use crate::output::list_items::pack::PackListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::pack;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn list(connection: &Connection, name: &Option<String>) -> Result<()> {
    let results = pack::query(connection, name.to_owned())?;
    let list_items = results
        .iter()
        .map(|value| PackListItem::from(connection, value))
        .collect::<Result<Vec<PackListItem>>>();

    TableOutput::new(list_items?).print();
    Ok(())
}
