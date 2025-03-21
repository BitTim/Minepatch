/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       func.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.03.25, 11:07
 */

use crate::comp::CompProcess;
use crate::db::Portable;
use crate::event;
use crate::prelude::*;
use bincode::config;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use std::sync::mpsc::Sender;

pub(crate) fn serialize<T>(tx: &Sender<Event>, obj: &T) -> Result<Vec<u8>>
where
    T: Portable,
{
    event::init_progress(tx, Process::Comp(CompProcess::Serialize), None)?;

    let bytes = bincode::encode_to_vec(obj, config::standard())?;
    event::end_progress(tx, Process::Comp(CompProcess::Serialize), None)?;

    Ok(bytes)
}

pub(crate) fn deserialize<T>(tx: &Sender<Event>, bytes: &[u8]) -> Result<T>
where
    T: Portable,
{
    event::init_progress(tx, Process::Comp(CompProcess::Deserialize), None)?;
    let (obj, _) = bincode::decode_from_slice(bytes, config::standard())?;
    event::end_progress(tx, Process::Comp(CompProcess::Deserialize), None)?;

    Ok(obj)
}

pub(crate) fn compress(tx: &Sender<Event>, data: &[u8]) -> Result<Vec<u8>> {
    event::init_progress(tx, Process::Comp(CompProcess::Compress), None)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;

    event::end_progress(tx, Process::Comp(CompProcess::Compress), None)?;
    Ok(compressed)
}

pub(crate) fn decompress(tx: &Sender<Event>, data: &[u8]) -> Result<Vec<u8>> {
    event::init_progress(tx, Process::Comp(CompProcess::Decompress), None)?;

    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;

    event::end_progress(tx, Process::Comp(CompProcess::Decompress), None)?;
    Ok(result)
}
