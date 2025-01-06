/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 01:23
 */
use crate::commands::vault::meta::Loader;
use crate::commands::vault::vault_error::VaultError;
use crate::common::error::ErrorType;
use crate::common::file::path_builder::PathBuilder;
use crate::common::{error, file};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub(crate) fn get_base_mod_dir_path() -> error::Result<PathBuf> {
    let dir = PathBuilder::new(&mut *file::get_data_path()?)
        .push("mods")
        .build();

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub(crate) fn build_mod_dir_path(
    mod_id: &str,
    loader: &str,
    filename: &str,
) -> error::Result<PathBuf> {
    let dir = PathBuilder::new(&mut *get_base_mod_dir_path()?)
        .push(mod_id)
        .push(loader)
        .build();

    fs::create_dir_all(&dir)?;

    let path = PathBuilder::new(&dir).push(filename).build();
    Ok(path)
}

pub(crate) fn detect_loader(path: &Path) -> error::Result<(Loader, String)> {
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
        Err(VaultError::NoLoaderDetected
            .builder()
            .context("File", &path.display().to_string())
            .build())
    } else {
        let (loader, mut meta_file) = result.unwrap();

        let mut data = String::new();
        meta_file.read_to_string(&mut data)?;

        Ok((loader, data))
    }
}
