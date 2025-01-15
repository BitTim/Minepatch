/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.01.25, 16:29
 */
use colored::Colorize;
use std::fmt::{Debug, Display};

pub mod detailed;
pub mod status;
pub mod table;

pub trait Output: Debug + Display {
    fn print(&self) {
        println!("{}", self)
    }
}

pub(crate) fn format_bool(value: &bool) -> String {
    match value {
        true => "✓ Yes".green().to_string(),
        false => "✗  No".red().to_string(),
    }
}

pub(crate) fn format_string_option(value: &Option<String>) -> String {
    match value {
        Some(value) => value.to_owned(),
        None => "?".red().to_string(),
    }
}
