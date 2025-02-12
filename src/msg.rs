/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 03:38
 */
use crate::hash::{HashMessage, HashProcess};
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::pack::{PackMessage, PackProcess};
use crate::patch::{PatchMessage, PatchProcess};
use crate::template::{TemplateMessage, TemplateProcess};
use crate::vault::{ModMessage, ModProcess};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Instance(InstanceProcess),
    Pack(PackProcess),
    Patch(PatchProcess),
    Template(TemplateProcess),
    Mod(ModProcess),
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Message {
    Hash(HashMessage),
    Instance(InstanceMessage),
    Pack(PackMessage),
    Patch(PatchMessage),
    Template(TemplateMessage),
    Mod(ModMessage),
}
