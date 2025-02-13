/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       remove.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.02.25, 03:16
 */
use crate::output::list_items::vault::ModListItem;
use inquire::Select;
use minepatch::prelude::*;
use minepatch::vault;
use minepatch::vault::VaultError;
use rusqlite::Connection;
use std::sync::mpsc::Sender;

pub(crate) fn remove(
    connection: &Connection,
    tx: &Sender<Event>,
    hash: &Option<String>,
    yes: bool,
    all: bool,
) -> Result<()> {
    vault::remove(connection, tx, hash.as_ref(), yes, all, |matches| {
        let options = matches
            .iter()
            .map(|entry| ModListItem::from(connection, tx, entry))
            .collect::<Vec<ModListItem>>();
        let result = Select::new(
            "Multiple entries match the provided hash. Please select the one you want to remove:",
            options,
        )
        .prompt()?;
        matches
            .iter()
            .find(|entry| entry.hash == result.hash)
            .ok_or(Error::Vault(VaultError::NotFound {
                hash: result.hash.to_owned(),
            }))
    })
}
