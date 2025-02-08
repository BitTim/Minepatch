/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       hash.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   08.02.25, 22:17
 */
use crate::prelude::*;
use sha256::Sha256Digest;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

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

pub(crate) fn hash_state_from_path(paths: &[PathBuf]) -> Result<String> {
    let hashes: HashSet<String> = paths
        .iter()
        .map(|mod_path| hash_file(mod_path))
        .collect::<Result<HashSet<String>>>()?;

    Ok(hash_state(&hashes))
}
