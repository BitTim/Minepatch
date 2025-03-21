/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:35
 */
use crate::output::list_items::instance::InstanceListItem;
use crate::output::table::TableOutput;
use mpcore::instance::InstanceRepo;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn list(conn: &Connection, tx: &Sender<Event>, name: &Option<String>) -> Result<()> {
    let instances = InstanceRepo::by_name_many(conn, name.to_owned().as_deref(), false)?
        .iter()
        .map(|instance| InstanceListItem::from(conn, tx, instance))
        .collect::<Vec<InstanceListItem>>();

    let output = TableOutput::new(instances, "No instances linked yet".to_owned()).to_string();
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
