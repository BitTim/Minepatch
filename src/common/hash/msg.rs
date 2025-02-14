/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.02.25, 03:30
 */
use std::path::PathBuf;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum HashProcess {
    HashFiles,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum HashMessage {
    HashFilesStatus { path: PathBuf },
}
