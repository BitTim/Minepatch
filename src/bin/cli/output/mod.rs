/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 14:01
 */
use colored::Colorize;
use std::fmt::{Debug, Display};

mod detailed;
mod displays;
mod status;
mod table;

pub trait _Output: Debug + Display {
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
