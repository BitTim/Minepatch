/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 19:26
 */
use crate::vault::data::Mod;
use std::path::PathBuf;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum ModProcess {
    Add,
    Remove,
    Validate,
    Export,
    Import,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum ModMessage {
    AddSuccess { value: Box<Mod> },
    RemoveStatus { hash: String },
    RemoveConfirm { value: Box<Mod> },
    ValidateSuccess { hash: String },
    RemoveSelect,
    RemoveOption { value: Box<Mod> },
    ValidateStatus { hash: String },
    ExportSuccess { hash: String, path: PathBuf },
    ImportSuccess { hash: String, path: PathBuf },
}
