/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       view.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
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

pub(crate) fn view(conn: &Connection, tx: &Sender<Event>, name: &str, bundle: &str) -> Result<()> {
    let patch = patch::query_single(conn, name, bundle)?;
    let relations = patch_with_mods::query_multiple(conn, name, bundle)?;
    let next_patch = patch::query_by_dependency_single(conn, name, bundle).ok();

    let (added_mod_relations, removed_mod_relations): (Vec<_>, Vec<_>) =
        relations.iter().partition(|rel| !rel.removed);

    let added_mods = query_mods(&added_mod_relations, conn)?;
    let removed_mods = query_mods(&removed_mod_relations, conn)?;

    let added_mods_table = TableOutput::new(
        added_mods
            .iter()
            .map(|value| ModListItem::from(conn, tx, value))
            .collect::<Vec<ModListItem>>(),
        "No mods added".bold().yellow().to_string(),
    );

    let removed_mods_table = TableOutput::new(
        removed_mods
            .iter()
            .map(|value| ModListItem::from(conn, tx, value))
            .collect::<Vec<ModListItem>>(),
        "No mods removed".bold().yellow().to_string(),
    );

    let valid = patch::validate(conn, tx, name, bundle, false).is_ok();
    let header_line = format!(
        "Patch '{}' for bundle '{}' ({})",
        name.cyan(),
        bundle.blue(),
        format_bool_valid(&valid)
    )
    .bold();

    let prev_patch_line = format!("Previous patch:\t\t'{}'", patch.dependency.purple());
    let next_patch_line = format!(
        "Next patch:\t\t'{}'",
        format_string_option(&next_patch.map(|value| value.name)).purple()
    );

    let output = format!(
        "\n{}\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n",
        header_line,
        prev_patch_line,
        next_patch_line,
        &"Added mods:".green().bold().underline().to_string(),
        added_mods_table,
        &"Removed mods:".red().bold().underline().to_string(),
        removed_mods_table
    );
    tx.send(Event::Log {
        message: Message::Transparent(output),
    })?;
    Ok(())
}

fn query_mods(relations: &[&PatchWithMods], conn: &Connection) -> Result<Vec<Mod>> {
    relations
        .iter()
        .map(|rel| vault::query_single(conn, &rel.mod_hash))
        .collect()
}
