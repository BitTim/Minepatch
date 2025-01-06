/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 14:11
 */
use crate::common::output::Output;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ListOutput {
    pub identifier: String,
    pub short_entries: Vec<Entry>,
    pub long_entries: Vec<Entry>,
}

impl Display for ListOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Details for ")
    }
}

impl Output for ListOutput {}
