/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 04:09
 */
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackProcess {
    Create,
    AddModFiles,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackMessage {
    AddModFileStatus(Vec<PackContext>),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PackContext {
    Path(PathBuf),
    Hash(String),
}
