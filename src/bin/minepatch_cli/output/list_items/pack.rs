/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       pack.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:12
 */
use crate::output::{format_bool, format_string_option};
use minepatch::pack;
use minepatch::pack::Pack;
use minepatch::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use tabled::Tabled;

#[derive(Tabled, Debug)]
pub struct PackListItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Template")]
    template: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

impl PackListItem {
    pub(crate) fn from(connection: &Connection, tx: &Sender<Event>, value: &Pack) -> Result<Self> {
        let valid = pack::validate(connection, tx, &value.name, false).is_ok();

        Ok(PackListItem {
            name: value.name.to_owned(),
            description: format_string_option(&value.description),
            template: format_string_option(&value.template),
            valid: format_bool(&valid),
        })
    }
}
