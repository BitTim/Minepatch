/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   22.01.25, 01:46
 */
use crate::output::detailed::{DetailedDisplayObject, DetailedOutput};
use crate::output::displays::vault::ModDisplay;
use crate::output::table::TableOutput;
use crate::output::Output;
use minepatch::prelude::*;
use minepatch::vault::query;
use rusqlite::Connection;

pub(crate) fn list(
    connection: &Connection,
    detailed: &bool,
    hash: &Option<String>,
    id: &Option<String>,
    name: &Option<String>,
) -> Result<()> {
    let results = query(
        connection,
        &hash.to_owned().unwrap_or_default(),
        &id.to_owned().unwrap_or_default(),
        &name.to_owned().unwrap_or_default(),
    )?;

    match *detailed {
        true => {
            let displays = results
                .iter()
                .map(DetailedDisplayObject::from)
                .collect::<Vec<DetailedDisplayObject>>();

            DetailedOutput::new(displays).print();
        }
        false => {
            let displays = results
                .iter()
                .map(ModDisplay::from)
                .collect::<Vec<ModDisplay>>();

            TableOutput::new(displays).print();
        }
    }

    Ok(())
}
