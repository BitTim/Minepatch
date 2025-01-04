/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   04.01.25, 23:55
 */
use crate::commands::vault::meta::Loader;
use crate::commands::vault::vault_error::VaultError;
use crate::commands::vault::vault_util::build_mod_dir_path;
use crate::commands::vault::Mod;
use crate::common::error::{CommonError, ErrorType};
use crate::common::file::error::FileError;
use crate::common::file::filename_from_path;
use crate::common::{error, file};
use colored::Colorize;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub fn add(path: &Path) -> error::Result<()> {
    let mut registry: Vec<Mod> = file::read_all()?;

    if !fs::exists(path)? {
        return Err(FileError::PathNotFound
            .builder()
            .context(&format!("File: '{}'", path.display()))
            .build());
    }

    let filename = filename_from_path(path)?;

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
    let mut archive = ZipArchive::new(&jar_file)?;

    let mut loaders = Loader::iter();
    let result = loop {
        match loaders.next() {
            Some(loader) => match archive.by_name(loader.meta_path()) {
                Ok(file) => break Some((loader, file)),
                Err(_) => continue,
            },
            None => break None,
        }
    };

    if result.is_none() {
        return Err(VaultError::NoLoaderDetected
            .builder()
            .context(&format!("File: '{}'", path.display()))
            .build());
    }

    let (loader, mut meta_file) = result.unwrap();

    let mut data = String::new();
    meta_file.read_to_string(&mut data)?;

    let meta = loader.extract_meta(&*data)?;
    let mod_file_path = build_mod_dir_path(&*meta.id, loader.name(), filename)?;

    drop(loader);
    drop(meta_file);
    drop(archive);
    drop(jar_file);

    fs::copy(path, &mod_file_path)?;
    fs::remove_file(path)?;

    registry.push(Mod::new(&*hash, &mod_file_path, meta));
    file::write_all(registry)?;

    println!(
        "{}Registered and moved mod into vault\n\t{}\n\t{}",
        "success: ".green().bold(),
        format!("File: '{}'", path.display()).cyan(),
        format!("Vault File: '{}'", mod_file_path.display()).cyan()
    );
    Ok(())
}
