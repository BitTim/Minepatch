/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 01:50
 */
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum HashProcess {
    HashFiles,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum HashMessage {
    HashFilesStatus { path: PathBuf },
}
