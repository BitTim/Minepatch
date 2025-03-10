/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 07:55
 */
use crate::bundle::Bundle;
use std::path::PathBuf;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum BundleProcess {
    Create,
    AddModFiles,
    Validate,
    Export,
    Import,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum BundleMessage {
    CreateSuccess { bundle: Box<Bundle> },
    AddModFileStatus { path: PathBuf, hash: String },
    ValidateSuccess { name: String },
    ValidateStatus { name: String },
    ExportSuccess { bundle: String, path: PathBuf },
}
