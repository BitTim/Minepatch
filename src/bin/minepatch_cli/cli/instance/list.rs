/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 19:36
 */
use crate::output::list_items::instance::InstanceListItem;
use crate::output::table::TableOutput;
use minepatch::instance;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &Option<String>,
) -> Result<()> {
    let instances = instance::query_multiple(connection, name.to_owned().as_deref())?
        .iter()
        .map(|instance| InstanceListItem::from(connection, tx, instance))
        .collect::<Vec<InstanceListItem>>();

    let output = TableOutput::new(instances, "No instances linked yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
