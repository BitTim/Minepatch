/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       hash.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   10.02.25, 18:37
 */
use crate::msg::Message;
use crate::prelude::*;
use crate::progress;
use crate::progress::event::Event;
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
    let id = progress::init_progress(tx, "Hashing mod files", Some(paths.len() as u64))?;
    let hashes: HashSet<String> = paths
        .par_iter()
        .map(|mod_path| {
            progress::tick_progress(tx, &id, Message::new(&mod_path.display().to_string()))?;
            hash_file(mod_path)
        })
        .collect::<Result<HashSet<String>>>()?;

    progress::end_progress(tx, id)?;
    Ok(hash_state(&hashes))
}
