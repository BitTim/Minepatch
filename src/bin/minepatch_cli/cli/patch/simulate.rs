/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:13
 */
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::Output;
use colored::Colorize;
use minepatch::prelude::*;
use minepatch::vault::Mod;
use minepatch::{patch, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn simulate(
    connection: &Connection,
    tx: &Sender<Event>,
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
        let hash = patch::simulate_dir_hash(connection, tx, name, pack)?;
        println!("Dir Hash: '{}'", hash.purple());
        return Ok(());
    }

    let mods = patch::simulate(connection, tx, name, pack)?
        .iter()
        .map(|hash| vault::query_single(connection, hash))
        .collect::<Result<Vec<Mod>>>()?;
    let displays = mods
        .iter()
        .map(|value| ModListItem::from(connection, tx, value))
        .collect::<Vec<ModListItem>>();

    TableOutput::new(displays, "No mods present in pack in simulation".to_owned()).print();
    Ok(())
}
