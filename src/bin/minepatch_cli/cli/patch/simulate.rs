/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.02.25, 02:49
 */
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use colored::Colorize;
use minepatch::msg::Message;
use minepatch::prelude::*;
use minepatch::vault::Mod;
use minepatch::{patch, vault};
use rusqlite::Connection;

pub(crate) fn simulate(connection: &Connection, name: &str, pack: &str) -> Result<()> {
    let mods = patch::simulate(connection, name, pack)?
        .iter()
        .map(|hash| vault::query_single(connection, hash))
        .collect::<Result<Vec<Mod>>>()?;
    let displays = mods
        .iter()
        .map(|value| ModListItem::from(connection, value))
        .collect::<Vec<ModListItem>>();

    let header_line = format!(
        "Simulation result for patch '{}' for pack '{}'",
        name.cyan(),
        pack.blue()
    )
    .bold();

    println!("{}", header_line);

    TableOutput::new(
        displays,
        Message::new("No mods present in pack in simulation"),
    )
    .print();
    Ok(())
}
