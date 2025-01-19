/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       detect_loader.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 18:24
 */

use crate::util::error;
use crate::util::error::ErrorType;
use crate::util::meta::data::Loader;
use crate::vault::error::VaultError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub(crate) fn detect_loader(
    path: &Path,
    silent: bool,
) -> error::Result<Option<(Loader, String, Option<String>)>> {
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

    if result.is_none() && !silent {
        println!(
            "{}",
            VaultError::NoLoaderDetected
                .builder()
                .context("File", &path.display().to_string())
                .build()
        );
    }

    Ok(result)
}
