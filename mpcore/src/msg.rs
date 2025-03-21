/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   21.03.25, 12:27
 */
use crate::bundle::{BundleMessage, BundleProcess};
use crate::comp::{CompMessage, CompProcess};
use crate::hash::{HashMessage, HashProcess};
use crate::instance::{InstanceMessage, InstanceProcess};
use crate::patch::{PatchMessage, PatchProcess};
use crate::vault::{ModMessage, ModProcess};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Process {
    Hash(HashProcess),
    Comp(CompProcess),
    Instance(InstanceProcess),
    Bundle(BundleProcess),
    Patch(PatchProcess),
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
    Mod(ModMessage),
}
