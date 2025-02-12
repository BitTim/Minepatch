/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:00
 */
use crate::vault::Mod;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ModProcess {
    Add,
    Remove,
    Validate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ModMessage {
    AddSuccess { value: Box<Mod> },
    RemoveStatus { hash: String },
    ValidateSuccess { hash: String },
}
