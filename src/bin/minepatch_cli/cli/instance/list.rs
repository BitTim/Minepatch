/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:19
 */
use crate::output::list_items::instance::InstanceListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::instance;
use minepatch::msg::Message;
use minepatch::prelude::*;
use rusqlite::Connection;

pub(crate) fn list(connection: &Connection, name: &Option<String>) -> Result<()> {
    let instances = instance::query_multiple(connection, name.to_owned().as_deref())?
        .iter()
        .map(|instance| InstanceListItem::from(connection, instance))
        .collect::<Vec<InstanceListItem>>();

    TableOutput::new(instances, Message::new("No instances linked yet")).print();
    Ok(())
}
