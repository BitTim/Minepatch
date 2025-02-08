/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       pack.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   07.02.25, 17:40
 */
use crate::output::{format_bool, format_string_option};
use minepatch::pack;
use minepatch::pack::Pack;
use minepatch::prelude::*;
use rusqlite::Connection;
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
    pub(crate) fn from(connection: &Connection, value: &Pack) -> Result<Self> {
        let valid = pack::validate(connection, &value.name, false).is_ok();

        Ok(PackListItem {
            name: value.name.to_owned(),
            description: format_string_option(&value.description),
            template: format_string_option(&value.template),
            valid: format_bool(&valid),
        })
    }
}
