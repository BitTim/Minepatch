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

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchProcess {
    Simulate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchMessage {
    SimulateStatus(Vec<PatchContext>),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchContext {
    Name(String),
}
