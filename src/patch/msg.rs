/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 01:51
 */

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchProcess {
    Simulate,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PatchMessage {
    SimulateStatus { name: String },
}
