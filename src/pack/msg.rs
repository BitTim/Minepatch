/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:45
 */
use crate::pack::Pack;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackProcess {
    Create,
    AddModFiles,
    Validate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackMessage {
    CreateSuccess { pack: Box<Pack> },
    AddModFileStatus { path: PathBuf, hash: String },
    ValidateSuccess { name: String },
}
