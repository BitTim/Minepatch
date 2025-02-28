/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:39
 */
use crate::bundle::{BundleMessage, BundleProcess};
use crate::hash::{HashMessage, HashProcess};
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::patch::{PatchMessage, PatchProcess};
use crate::template::{TemplateMessage, TemplateProcess};
use crate::vault::{ModMessage, ModProcess};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Instance(InstanceProcess),
    Bundle(BundleProcess),
    Patch(PatchProcess),
    Template(TemplateProcess),
    Mod(ModProcess),
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Message {
    Transparent(String),
    Hash(HashMessage),
    Instance(InstanceMessage),
    Bundle(BundleMessage),
    Patch(PatchMessage),
    Template(TemplateMessage),
    Mod(ModMessage),
}
