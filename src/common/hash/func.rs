/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       func.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:53
 */
use crate::common::msg;
use crate::common::msg::Event;
use crate::hash::msg::{HashContext, HashMessage, HashProcess};
use crate::prelude::*;
use rayon::prelude::*;
use sha256::Sha256Digest;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

pub(crate) fn hash_state(hashes: &HashSet<String>) -> String {
    let mut hashes = Vec::from_iter(hashes.to_owned());
    hashes.sort();

    let data = hashes.join("\n");
    data.digest()
}

pub(crate) fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::OpenOptions::new().read(true).open(&path)?;

    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data)?;

    Ok(data.digest())
}

pub(crate) fn hash_state_from_path(tx: &Sender<Event>, paths: &[PathBuf]) -> Result<String> {
    msg::init_progress(
        tx,
        Process::Hash(HashProcess::HashFiles),
        Some(paths.len() as u64),
    )?;

    let hashes: HashSet<String> = paths
        .par_iter()
        .map(|mod_path| {
            msg::tick_progress(
                tx,
                Process::Hash(HashProcess::HashFiles),
                Message::Hash(HashMessage::HashFilesStatus(vec![HashContext::Path(
                    mod_path.to_path_buf(),
                )])),
            )?;
            hash_file(mod_path)
        })
        .collect::<Result<HashSet<String>>>()?;

    msg::end_progress(tx, Process::Hash(HashProcess::HashFiles))?;
    Ok(hash_state(&hashes))
}
