/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       vault_util.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   06.01.25, 18:17
 */
use crate::commands::vault::meta::Loader;
use crate::commands::vault::vault_error::VaultError;
use crate::commands::vault::Mod;
use crate::common::error::ErrorType;
use crate::common::file::path_builder::PathBuilder;
use crate::common::{error, file};
use inquire::Confirm;
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

pub(crate) fn detect_loader(path: &Path) -> error::Result<(Loader, String, Option<String>)> {
    let jar_file = File::open(path)?;
    let mut archive = ZipArchive::new(&jar_file)?;

    let mut loaders = Loader::iter();
    let result = loop {
        let (loader, data) = match loaders.next() {
            Some(loader) => match archive.by_name(loader.meta_path()) {
                Ok(mut file) => {
                    let mut data = String::new();
                    file.read_to_string(&mut data)?;

                    (loader, data.to_owned())
                }
                Err(_) => continue,
            },
            None => break None,
        };

        let extra_path = loader.extra_path();
        let extra = if let Some(path) = extra_path {
            match archive.by_name(path) {
                Ok(mut file) => {
                    let mut extra = String::new();
                    file.read_to_string(&mut extra)?;

                    Some(extra)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        break Some((loader, data, extra));
    };

    if result.is_none() {
        Err(VaultError::NoLoaderDetected
            .builder()
            .context("File", &path.display().to_string())
            .build())
    } else {
        Ok(result.unwrap())
    }
}

pub fn check_entry<'a>(registry: &'a [Mod], hash: &str) -> error::Result<(usize, &'a Mod)> {
    match registry
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.hash == hash)
    {
        Some((index, entry)) => Ok((index, entry)),
        None => Err(VaultError::HashNotFound
            .builder()
            .context("Hash", hash)
            .build()),
    }
}

pub(crate) fn confirm_remove(entry: &Mod) -> error::Result<bool> {
    let ans = Confirm::new(&format!(
        "Do you really want to remove '{}' ({}, {}) from the vault?",
        entry.meta.name, entry.meta.version, entry.meta.loader
    ))
    .with_default(false)
    .with_help_message(&format!("Hash: '{}'", &entry.hash))
    .prompt()?;

    Ok(ans)
}
