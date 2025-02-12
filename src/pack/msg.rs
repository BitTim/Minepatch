/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 02:55
 */
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackProcess {
    Create,
    AddModFiles,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackMessage {
    CreateSuccess { name: String },
    AddModFileStatus { path: PathBuf, hash: String },
}
