/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 02:28
 */
use crate::instance::Instance;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum InstanceProcess {
    Detect,
    Link,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum InstanceMessage {
    DetectSuccess { pack: String, patch: String },
    LinkSuccess { instance: Instance },
}
