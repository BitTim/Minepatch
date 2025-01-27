/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.01.25, 09:58
 */
use crate::output::list_items::template::TemplateListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::prelude::*;
use minepatch::template;
use rusqlite::Connection;

pub(crate) fn list(connection: &Connection, name: &Option<String>) -> Result<()> {
    let templates = template::query(connection, name.to_owned().as_deref())?
        .iter()
        .map(TemplateListItem::from)
        .collect::<Vec<TemplateListItem>>();

    TableOutput::new(templates).print();
    Ok(())
}
