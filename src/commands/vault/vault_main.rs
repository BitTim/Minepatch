/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   03.01.25, 23:56
 */
use crate::commands::vault::meta::Loader;
use crate::commands::vault::vault_error::VaultError;
use crate::commands::vault::Mod;
use crate::common::error::{CommonError, ErrorType};
use crate::common::file::error::FileError;
use crate::common::{error, file};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub fn add(path: &Path) -> error::Result<()> {
    // 1. [x] Get registry
    // 2. [x] Hash file
    // 2. [x] Open file from path as zip
    // 4. [x] Extract metadata
    // 5. [ ] Move file to proper folder (dictated by metadata)
    // 6. [ ] Add to registry
    // 7. [ ] Save registry

    let registry: Vec<Mod> = file::read_all()?;
    println!("{:?}", registry); // FIXME: Temporary to suppress warning

    if !fs::exists(path)? {
        return Err(FileError::PathNotFound
            .builder()
            .context(&format!("File: '{}'", path.display()))
            .build());
    }

    let hash = match sha256::try_digest(path) {
        Ok(value) => value,
        Err(error) => {
            return Err(CommonError::Wrapper(Box::new(error))
                .builder()
                .context("Error during file hashing")
                .context(&format!("File: '{}'", path.display()))
                .build())
        }
    };

    let jar_file = File::open(path)?;
    let mut archive = ZipArchive::new(jar_file)?;

    let mut detected = false;
    for loader in Loader::iter() {
        let mut meta_file = match archive.by_name(loader.meta_path()).ok() {
            None => {
                continue;
            }
            Some(file) => file,
        };

        let mut data = String::new();
        meta_file.read_to_string(&mut data)?;

        let meta = loader.extract_meta(&*data)?;
        println!("{:?}", meta);

        detected = true;
        break;
    }

    if !detected {
        return Err(VaultError::NoLoaderDetected
            .builder()
            .context(&format!("File: '{}'", path.display()))
            .build());
    }

    println!("{}", hash);

    Ok(())
}
