/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.02.25, 01:48
 */
use crate::vault::Mod;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum ModProcess {
    Add,
    Remove,
    Validate,
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
}
