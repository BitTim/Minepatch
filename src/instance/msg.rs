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
use crate::instance::Instance;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum InstanceProcess {
    Detect,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum InstanceMessage {
    LinkSuccess(Vec<InstanceContext>),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum InstanceContext {
    SuccessObj(Instance),
}
