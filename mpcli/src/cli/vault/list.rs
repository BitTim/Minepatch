/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:50
 */
use crate::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use mpcore::hash::Hash;
use mpcore::prelude::*;
use mpcore::vault::VaultRepo;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    conn: &Connection,
    tx: &Sender<Event>,
    detailed: &bool,
    hash: &Option<Hash>,
    id: &Option<String>,
    name: &Option<String>,
) -> Result<()> {
    let results = VaultRepo::by_hash_and_id_and_name_many(
        conn,
        hash.to_owned().as_ref(),
        id.to_owned().as_deref(),
        name.to_owned().as_deref(),
        false,
    )?;

    let output = match *detailed {
        true => {
            let displays = results
                .iter()
                .map(|value| DetailedDisplayObject::from_mod(conn, value))
                .collect::<Vec<DetailedDisplayObject>>();

            DetailedOutput::new(displays).to_string()
        }
        false => {
            let displays = results
                .iter()
                .map(|value| ModListItem::from(conn, value))
                .collect::<Vec<ModListItem>>();

            TableOutput::new(displays, "No mods added to vault yet".to_owned()).to_string()
        }
    };

    Ok(tx.send(Event::Log {
        message: Message::Transparent(output),
    })?)
}
