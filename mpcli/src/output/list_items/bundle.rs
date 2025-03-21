/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       bundle.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 12:28
 */
use crate::output::{format_bool, format_string_option};
use mpcore::bundle;
use mpcore::bundle::Bundle;
use mpcore::prelude::*;
use rusqlite::Connection;
use std::sync::mpsc::Sender;
use tabled::Tabled;

#[derive(Tabled, Debug)]
pub struct PackListItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Valid")]
    valid: String,
}

impl PackListItem {
    pub(crate) fn from(conn: &Connection, tx: &Sender<Event>, value: &Bundle) -> Result<Self> {
        let valid = bundle::validate(conn, tx, &value.name, false).is_ok();

        Ok(PackListItem {
            name: value.name.to_owned(),
            description: format_string_option(&value.description),
            valid: format_bool(&valid),
        })
    }
}
