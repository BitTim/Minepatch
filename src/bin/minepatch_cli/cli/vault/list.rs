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
use crate::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use minepatch::prelude::*;
use minepatch::vault::query_multiple;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    conn: &Connection,
    tx: &Sender<Event>,
    detailed: &bool,
    hash: &Option<String>,
    id: &Option<String>,
    name: &Option<String>,
) -> Result<()> {
    let results = query_multiple(
        conn,
        hash.to_owned().as_deref(),
        id.to_owned().as_deref(),
        name.to_owned().as_deref(),
    )?;

    let output = match *detailed {
        true => {
            let displays = results
                .iter()
                .map(|value| DetailedDisplayObject::from_mod(conn, tx, value))
                .collect::<Vec<DetailedDisplayObject>>();

            DetailedOutput::new(displays).to_string()
        }
        false => {
            let displays = results
                .iter()
                .map(|value| ModListItem::from(conn, tx, value))
                .collect::<Vec<ModListItem>>();

            TableOutput::new(displays, "No mods added to vault yet".to_owned()).to_string()
        }
    };

    Ok(tx.send(Event::Log {
        message: Message::Transparent(output),
    })?)
}
