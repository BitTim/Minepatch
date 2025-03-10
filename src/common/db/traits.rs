/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       traits.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.03.25, 10:25
 */
use crate::error::Error;
use crate::prelude::*;
use crate::{comp, event, file};
use rusqlite::{Row, ToSql};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::hash::Hash;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) trait Entity: Eq + PartialEq + Hash {
    fn table_name() -> String;
    fn from_row(row: &Row) -> Result<Box<Self>>;
    fn to_params(&self) -> Vec<Box<dyn ToSql>>;
}

pub(crate) trait Portable:
    Eq + PartialEq + Hash + Serialize + for<'de> Deserialize<'de>
{
    fn file_extension() -> String;
    fn object_name(&self) -> String;
    fn export(
        &self,
        tx: &Sender<Event>,
        path: &Path,
        process: Process,
        success_msg: Option<Message>,
    ) -> Result<()> {
        event::init_progress(tx, process.to_owned(), None)?;

        let path = file::canonicalize_entity_path(path.to_owned(), self)?;
        let serialized = bincode::serialize(self)?;
        let compressed = comp::compress(&serialized)?;

        let mut file = File::create_new(&path)?;
        file.write_all(&compressed)?;

        event::end_progress(tx, process, success_msg)?;
        Ok(())
    }

    fn import(
        tx: &Sender<Event>,
        path: &Path,
        process: Process,
        success_msg: Option<Message>,
    ) -> Result<Self> {
        event::init_progress(tx, process.to_owned(), None)?;

        let mut file = File::open(path)?;
        let mut data: Vec<u8> = vec![];

        file.read_to_end(&mut data)?;

        let decompressed = comp::decompress(&data)?;
        let deserialized = bincode::deserialize::<Self>(&decompressed)?;

        event::end_progress(tx, process, success_msg)?;
        Ok(deserialized)
    }
}

pub(crate) trait Filter {
    fn value(&self) -> String;
    fn params(&self) -> Vec<Box<dyn ToSql>>;
    fn error(&self) -> Error;
}

pub(crate) trait InsertableFilter<T> {
    fn insert(value: T) -> Self;
}
