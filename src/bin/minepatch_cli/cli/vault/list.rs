/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.02.25, 21:41
 */
use crate::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::vault::query_multiple;
use rusqlite::Connection;

pub(crate) fn list(
    connection: &Connection,
    detailed: &bool,
    hash: &Option<String>,
    id: &Option<String>,
    name: &Option<String>,
) -> Result<()> {
    let results = query_multiple(
        connection,
        hash.to_owned().as_deref(),
        id.to_owned().as_deref(),
        name.to_owned().as_deref(),
    )?;

    match *detailed {
        true => {
            let displays = results
                .iter()
                .map(|value| DetailedDisplayObject::from_mod(connection, value))
                .collect::<Vec<DetailedDisplayObject>>();

            DetailedOutput::new(displays).print();
        }
        false => {
            let displays = results
                .iter()
                .map(|value| ModListItem::from(connection, value))
                .collect::<Vec<ModListItem>>();

            TableOutput::new(displays, Message::new("No mods added to vault yet")).print();
        }
    }

    Ok(())
}
