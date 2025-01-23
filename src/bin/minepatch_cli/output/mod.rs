/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:20
 */
use colored::Colorize;
use std::fmt::{Debug, Display};

pub(crate) mod detailed;
pub(crate) mod displays;
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

pub fn format_string_option(value: &Option<String>) -> String {
    match value {
        Some(value) => value.to_owned(),
        None => "?".red().to_string(),
    }
}
