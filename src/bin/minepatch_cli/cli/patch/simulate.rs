/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       simulate.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:57
 */
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use colored::Colorize;
use minepatch::prelude::*;
use minepatch::vault::Mod;
use minepatch::{patch, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn simulate(
    conn: &Connection,
    tx: &Sender<Event>,
    name: &str,
    bundle: &str,
    dir_hash: &bool,
) -> Result<()> {
    let header_line = format!(
        "Simulation result for patch '{}' for bundle '{}'",
        name.cyan(),
        bundle.blue()
    )
    .bold();

    if *dir_hash {
        let hash = patch::simulate_dir_hash(conn, tx, name, bundle)?;
        let hash_line = format!("Dir Hash: '{}'", hash.purple());
        let output = format!("{}\n{}", header_line, hash_line);
        tx.send(Event::Log {
            message: Message::Transparent(output),
        })?;

        return Ok(());
    }

    let mods = patch::simulate(conn, tx, name, bundle)?
        .iter()
        .map(|hash| vault::query_single(conn, hash))
        .collect::<Result<Vec<Mod>>>()?;
    let displays = mods
        .iter()
        .map(|value| ModListItem::from(conn, tx, value))
        .collect::<Vec<ModListItem>>();

    let output = format!(
        "{}\n{}",
        header_line,
        TableOutput::new(
            displays,
            "No mods present in bundle in simulation".to_owned()
        )
        .to_string()
    );
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}
