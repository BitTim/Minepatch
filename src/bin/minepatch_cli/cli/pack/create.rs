/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 17:33
 */
use minepatch::pack;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    description: &Option<String>,
    template: &Option<String>,
    from: &Option<String>,
    instance: &Option<String>,
) -> Result<()> {
    pack::create(
        connection,
        tx,
        name,
        description.as_deref(),
        template.as_deref(),
        from.as_deref(),
        instance.as_deref(),
    )
}
