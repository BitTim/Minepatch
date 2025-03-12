/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       common.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 18:55
 */
use crate::common::meta::data::Meta;
use crate::common::meta::error::MetaError;
use crate::file::build_vault_path;
use crate::meta::data::Loader;
use crate::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub(crate) fn extract_meta_json<F>(
    data: &str,
    loader: &str,
    filename: &str,
    meta_from_obj: F,
) -> Result<Meta>
where
    F: FnOnce(&Value, &str, &str) -> Option<Meta>,
{
    meta_from_obj(&parse_json(data)?, loader, filename)
        .ok_or(Error::Meta(MetaError::MalformedMetaFile(loader.to_owned())))
}

pub(crate) fn parse_json(data: &str) -> Result<Value> {
    let mut trimmed_data = data.to_owned();
    trimmed_data.retain(|c| c != '\n');

    Ok(serde_json::from_str(&trimmed_data)?)
}

pub(crate) fn extract_string_json(obj: &Value, key: &str) -> Option<String> {
    Some(obj.get(key)?.as_str()?.trim().to_owned())
}

pub(crate) fn detect_loader(path: &Path) -> Result<Option<(Loader, String, Option<String>)>> {
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

    Ok(result)
}

pub(crate) fn extract_meta(
    loader_result: Option<(Loader, String, Option<String>)>,
    filename: &str,
) -> Result<(Meta, PathBuf)> {
    Ok(match loader_result {
        Some((loader, data, extra)) => {
            let meta = loader.extract_meta(&data, &extra, filename)?;
            let mod_file_path = build_vault_path(&meta.id, &meta.loader, filename)?;
            (meta, mod_file_path)
        }
        None => {
            let meta = Meta::empty(filename);
            let mod_file_path = build_vault_path(&meta.id, &meta.loader, filename)?;
            (meta, mod_file_path)
        }
    })
}
