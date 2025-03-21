/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       view.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 14:50
 */
use crate::output::list_items::vault::ModListItem;
use crate::output::table::TableOutput;
use crate::output::{format_bool_valid, format_string_option};
use colored::Colorize;
use mpcore::patch::PatchRepo;
use mpcore::patch_with_mods::PatchModRelation;
use mpcore::prelude::*;
use mpcore::vault::{Mod, VaultRepo};
use mpcore::{patch, patch_with_mods};
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn view(conn: &Connection, tx: &Sender<Event>, name: &str, bundle: &str) -> Result<()> {
    // TODO: Handle "exact" value
    let patch = PatchRepo::by_name(conn, name, bundle, true)?;
    let relations = patch_with_mods::query_multiple(conn, name, bundle)?;
    let next_patch = PatchRepo::by_dependency(conn, name, bundle, true).ok();

    let (added_mod_relations, removed_mod_relations): (Vec<_>, Vec<_>) =
        relations.iter().partition(|rel| !rel.removed);

    let added_mods = query_mods(&added_mod_relations, conn)?;
    let removed_mods = query_mods(&removed_mod_relations, conn)?;

    let added_mods_table = TableOutput::new(
        added_mods
            .iter()
            .map(|value| ModListItem::from(conn, value))
            .collect::<Vec<ModListItem>>(),
        "No mods added".bold().yellow().to_string(),
    );

    let removed_mods_table = TableOutput::new(
        removed_mods
            .iter()
            .map(|value| ModListItem::from(conn, value))
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

fn query_mods(relations: &[&PatchModRelation], conn: &Connection) -> Result<Vec<Mod>> {
    relations
        .iter()
        .map(|rel| VaultRepo::by_hash(conn, &rel.mod_hash, true))
        .collect()
}
