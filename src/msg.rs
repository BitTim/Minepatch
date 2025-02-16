/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.02.25, 01:41
 */
use crate::hash::{HashMessage, HashProcess};
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::pack::{PackMessage, PackProcess};
use crate::patch::{PatchMessage, PatchProcess};
use crate::template::{TemplateMessage, TemplateProcess};
use crate::vault::{ModMessage, ModProcess};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Instance(InstanceProcess),
    Pack(PackProcess),
    Patch(PatchProcess),
    Template(TemplateProcess),
    Mod(ModProcess),
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Message {
    Transparent(String),
    Hash(HashMessage),
    Instance(InstanceMessage),
    Pack(PackMessage),
    Patch(PatchMessage),
    Template(TemplateMessage),
    Mod(ModMessage),
}
