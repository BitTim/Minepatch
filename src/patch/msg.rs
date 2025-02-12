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
use crate::patch::Patch;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchProcess {
    Create,
    Exclude,
    Generate,
    HashModFiles,
    Include,
    Simulate,
    Validate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchMessage {
    CreateSuccess { patch: Box<Patch> },
    ExcludeSuccess { hash: String },
    GenerateSuccess { name: String, instance: String },
    HashModFileStatus { path: PathBuf, hash: String },
    IncludeSuccess { hash: String },
    SimulateStatus { name: String },
    ValidateSuccess { name: String },
}
