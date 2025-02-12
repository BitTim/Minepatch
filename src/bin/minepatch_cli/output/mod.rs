/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.02.25, 21:00
 */
use colored::Colorize;
use std::fmt::{Debug, Display};

pub(crate) mod detailed;
pub(crate) mod list_items;
pub(crate) mod status;
pub(crate) mod table;

pub trait Output: Debug + Display {
    fn print(&self) {
        println!("{}", self)
    }
}

pub fn format_bool(value: &bool) -> String {
    match value {
        true => "✓ Yes".green().to_string(),
        false => "✗  No".red().to_string(),
    }
}
pub fn format_bool_valid(value: &bool) -> String {
    match value {
        true => "✓   Valid".green().to_string(),
        false => "✗ Invalid".red().to_string(),
    }
}

pub fn format_string_option(value: &Option<String>) -> String {
    match value {
        Some(value) => value.to_owned(),
        None => "?".red().to_string(),
    }
}
