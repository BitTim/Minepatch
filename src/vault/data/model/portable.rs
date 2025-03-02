/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       portable.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.03.25, 16:24
 */
use crate::db::Portable;
use crate::file;
use crate::meta::data::Meta;
use crate::prelude::*;
use crate::vault::data::Mod;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct PortableMod {
    pub hash: String,
    pub meta: Meta,
    pub filename: String,
    pub jar_binary: Vec<u8>,
}

impl PortableMod {
    pub fn new(value: Mod) -> Result<Self> {
        let mut jar_file = File::open(&value.path)?;
        let mut jar_binary = vec![];
        jar_file.read_to_end(&mut jar_binary)?;

        Ok(Self {
            hash: value.hash,
            meta: value.meta,
            filename: file::filename_from_path(&value.path)?.to_owned(),
            jar_binary,
        })
    }
}

impl Portable<'_> for PortableMod {
    fn file_extension() -> String {
        "mpm".to_owned()
    }

    fn object_name(&self) -> String {
        self.meta.id.to_owned()
    }
}
