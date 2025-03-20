/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       portable.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:45
 */
use crate::db::{Portable, Repo};
use crate::file;
use crate::file::build_vault_path;
use crate::legacy::vault::data::{Mod, VaultRepo};
use crate::meta::data::Meta;
use crate::prelude::*;
use bincode::{Decode, Encode};
use rusqlite::Connection;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Encode, Decode)]
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

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        let new_vault_path = build_vault_path(&self.meta.id, &self.meta.loader, &self.filename)?;

        let mut jar = File::create_new(&new_vault_path)?;
        jar.write_all(&self.jar_binary)?;

        let hash = self.hash.to_owned();
        VaultRepo::insert(conn, Mod::new(&hash, &new_vault_path, self.meta.to_owned()))?;
        Ok(())
    }
}

impl Portable for PortableMod {
    fn file_extension() -> String {
        "mpm".to_owned()
    }

    fn object_name(&self) -> String {
        self.meta.id.to_owned()
    }
}
