/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:31
 */
use crate::hash::{HashMessage, HashProcess};
use crate::legacy::bundle::{BundleMessage, BundleProcess};
use crate::legacy::common::comp::{CompMessage, CompProcess};
use crate::legacy::instance::{InstanceMessage, InstanceProcess};
use crate::legacy::patch::{PatchMessage, PatchProcess};
use crate::legacy::template::{TemplateMessage, TemplateProcess};
use crate::legacy::vault::{ModMessage, ModProcess};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Comp(CompProcess),
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
    Comp(CompMessage),
    Instance(InstanceMessage),
    Bundle(BundleMessage),
    Patch(PatchMessage),
    Template(TemplateMessage),
    Mod(ModMessage),
}
