/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       hash.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   23.01.25, 17:11
 */
use crate::prelude::*;
use sha256::Sha256Digest;
use std::fs;
use std::io::Read;
use std::path::Path;

pub(crate) fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::OpenOptions::new().read(true).open(&path)?;

    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data)?;

    Ok(data.digest())
}

pub(crate) fn hash_state(hashes: &[String]) -> String {
    let data = hashes.join("\n");
    data.digest()
}
