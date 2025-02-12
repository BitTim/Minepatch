/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       view.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:13
 */
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::{format_bool_valid, format_string_option};
use colored::Colorize;
use minepatch::patch_with_mods::PatchWithMods;
use minepatch::prelude::*;
use minepatch::vault::Mod;
use minepatch::{patch, patch_with_mods, vault};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn view(
    connection: &Connection,
    tx: &Sender<Event>,
    name: &str,
    pack: &str,
) -> Result<()> {
    let patch = patch::query_single(connection, name, pack)?;
    let relations = patch_with_mods::query_multiple(connection, name, pack)?;
    let next_patch = patch::query_by_dependency_single(connection, name, pack).ok();

    let (added_mod_relations, removed_mod_relations): (Vec<_>, Vec<_>) =
        relations.iter().partition(|rel| !rel.removed);

    let added_mods = query_mods(&added_mod_relations, connection)?;
    let removed_mods = query_mods(&removed_mod_relations, connection)?;

    let added_mods_table = TableOutput::new(
        added_mods
            .iter()
            .map(|value| ModListItem::from(connection, tx, value))
            .collect::<Vec<ModListItem>>(),
        "No mods added".bold().yellow().to_string(),
    );

    let removed_mods_table = TableOutput::new(
        removed_mods
            .iter()
            .map(|value| ModListItem::from(connection, tx, value))
            .collect::<Vec<ModListItem>>(),
        "No mods removed".bold().yellow().to_string(),
    );

    let valid = patch::validate(connection, tx, name, pack, false).is_ok();
    let header_line = format!(
        "Patch '{}' for pack '{}' ({})",
        name.cyan(),
        pack.blue(),
        format_bool_valid(&valid)
    )
    .bold();

    let prev_patch_line = format!("Previous patch:\t\t'{}'", patch.dependency.purple());
    let next_patch_line = format!(
        "Next patch:\t\t'{}'",
        format_string_option(&next_patch.map(|value| value.name)).purple()
    );

    println!(
        "\n{}\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n",
        header_line,
        prev_patch_line,
        next_patch_line,
        &"Added mods:".green().bold().underline().to_string(),
        added_mods_table,
        &"Removed mods:".red().bold().underline().to_string(),
        removed_mods_table
    );
    Ok(())
}

fn query_mods(relations: &[&PatchWithMods], connection: &Connection) -> Result<Vec<Mod>> {
    relations
        .iter()
        .map(|rel| vault::query_single(connection, &rel.mod_hash))
        .collect()
}
