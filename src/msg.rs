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
use crate::hash::{HashContext, HashMessage, HashProcess};
use crate::instance::{InstanceContext, InstanceMessage, InstanceProcess};
use crate::pack::{PackContext, PackMessage, PackProcess};
use crate::patch::{PatchContext, PatchMessage, PatchProcess};
use crate::vault::{ModContext, ModMessage, ModProcess};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Instance(InstanceProcess),
    Pack(PackProcess),
    Patch(PatchProcess),
    Mod(ModProcess),
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Message {
    Hash(HashMessage),
    Instance(InstanceMessage),
    Pack(PackMessage),
    Patch(PatchMessage),
    Mod(ModMessage),
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Context {
    Hash(HashContext),
    Instance(InstanceContext),
    Pack(PackContext),
    Patch(PatchContext),
    Mod(ModContext),
}
