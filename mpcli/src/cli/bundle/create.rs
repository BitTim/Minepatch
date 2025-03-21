/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       create.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:17
 */
use mpcore::bundle;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::path;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) fn create(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    description: &Option<String>,
    template: &Option<String>,
    from: Option<&Path>,
    instance: &Option<String>,
) -> Result<()> {
    bundle::create(
        conn,
        tx,
        name,
        description.as_deref(),
        template.as_deref(),
        from.map(|path| path::absolute(path)?.canonicalize())
            .transpose()?
            .as_deref(),
        instance.as_deref(),
    )
}
