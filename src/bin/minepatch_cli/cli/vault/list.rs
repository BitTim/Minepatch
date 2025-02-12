/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:13
 */
use crate::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::prelude::*;
use minepatch::vault::query_multiple;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    connection: &Connection,
    tx: &Sender<Event>,
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
                .map(|value| DetailedDisplayObject::from_mod(connection, tx, value))
                .collect::<Vec<DetailedDisplayObject>>();

            DetailedOutput::new(displays).print();
        }
        false => {
            let displays = results
                .iter()
                .map(|value| ModListItem::from(connection, tx, value))
                .collect::<Vec<ModListItem>>();

            TableOutput::new(displays, "No mods added to vault yet".to_owned()).print();
        }
    }

    Ok(())
}
