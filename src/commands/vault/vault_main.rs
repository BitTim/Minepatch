/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.01.25, 22:25
 */
use crate::commands::vault::Mod;
use crate::common::error::{CommonError, ErrorType};
use crate::common::file::error::FileError;
use crate::common::{error, file};
use std::fs;
use std::path::Path;

pub fn add(path: &Path) -> error::Result<()> {
    // 1. Get registry
    // 2. Hash file
    // 2. Open file from path as zip
    // 4. Extract metadata
    // 5. Move file to proper folder (dictated by metadata)
    // 6. Add to registry
    // 7. Save registry

    let _: Vec<Mod> = file::read_all()?;

    if !fs::exists(path)? {
        return Err(FileError::PathNotFound
            .builder()
            .context(&path.display().to_string())
            .build());
    }

    let hash = match sha256::try_digest(path) {
        Ok(value) => value,
        Err(error) => {
            return Err(CommonError::Wrapper(Box::new(error))
                .builder()
                .context("Error during file hashing")
                .context(&path.display().to_string())
                .build())
        }
    };

    println!("{}", hash);

    Ok(())
}
