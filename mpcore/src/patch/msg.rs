/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.03.25, 11:26
 */
use crate::patch::Patch;
use std::path::PathBuf;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum PatchProcess {
    Create,
    Exclude,
    Generate,
    HashModFiles,
    Include,
    Simulate,
    Validate,
    Delete,
    Rename,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum PatchMessage {
    CreateSuccess {
        patch: Box<Patch>,
    },
    ExcludeSuccess {
        hash: String,
    },
    GenerateSuccess {
        name: String,
        instance: String,
    },
    HashModFileStatus {
        path: PathBuf,
        hash: String,
    },
    IncludeSuccess {
        hash: String,
    },
    SimulateStatus {
        name: String,
    },
    ValidateSuccess {
        name: String,
    },
    ValidateStatus {
        bundle: String,
        name: String,
    },
    DeleteSuccess {
        name: String,
        bundle: String,
    },
    RenameSuccess {
        name: String,
        bundle: String,
        new_name: String,
    },
}
