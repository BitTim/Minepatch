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
use crate::instance::Instance;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum InstanceProcess {
    Apply,
    Detect,
    Link,
    Validate,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum InstanceMessage {
    ApplySuccess { pack: String, patch: String },
    DetectSuccess { pack: String, patch: String },
    LinkSuccess { instance: Box<Instance> },
    ValidateSuccess { name: String },
}
