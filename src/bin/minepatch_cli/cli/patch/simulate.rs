/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 11:19
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

pub(crate) fn simulate(
    connection: &Connection,
    name: &str,
    pack: &str,
    dir_hash: &bool,
) -> Result<()> {
    let header_line = format!(
        "Simulation result for patch '{}' for pack '{}'",
        name.cyan(),
        pack.blue()
    )
    .bold();
    println!("{}", header_line);

    if *dir_hash {
        let hash = patch::simulate_dir_hash(connection, name, pack)?;
        println!("Dir Hash: '{}'", hash.purple());
        return Ok(());
    }

    let mods = patch::simulate(connection, name, pack)?
        .iter()
        .map(|hash| vault::query_single(connection, hash))
        .collect::<Result<Vec<Mod>>>()?;
    let displays = mods
        .iter()
        .map(|value| ModListItem::from(connection, value))
        .collect::<Vec<ModListItem>>();

    TableOutput::new(
        displays,
        Message::new("No mods present in pack in simulation"),
    )
    .print();
    Ok(())
}
