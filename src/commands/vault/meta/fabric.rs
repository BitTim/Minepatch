/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       fabric.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:22
 */
use crate::commands::vault::meta::meta_error::MetaError;
use crate::commands::vault::meta::Meta;
use crate::common::error;
use crate::common::error::ErrorType;
use serde_json::Value;

pub(crate) fn extract_meta(data: &str, loader: &str) -> error::Result<Meta> {
    let obj: Value = serde_json::from_str(data)?;
    let meta = meta_from_obj(&obj, loader).ok_or_else(|| {
        MetaError::MalformedMetaFile
            .builder()
            .context("Detected loader", loader)
            .build()
    })?;
    Ok(meta)
}

fn extract_string(obj: &Value, key: &str) -> Option<String> {
    Some(obj.get(key)?.as_str()?.to_owned())
}

fn extract_string_list(obj: &Value, key: &str) -> Option<Vec<String>> {
    Some(
        obj.get(key)?
            .as_array()?
            .iter()
            .filter_map(|author| author.as_str().map(|author| author.to_owned()))
            .collect(),
    )
}

fn meta_from_obj(obj: &Value, loader: &str) -> Option<Meta> {
    let mod_id = extract_string(obj, "id")?;

    Some(Meta {
        id: mod_id,
        name: extract_string(obj, "name")?,
        version: extract_string(obj, "version")?,
        description: extract_string(obj, "description")?,
        authors: extract_string_list(obj, "authors")?,
        loader: loader.to_owned(),
        loader_version: extract_string(obj.get("depends")?, "fabricloader"),
        minecraft_version: extract_string(obj.get("depends")?, "minecraft"),
    })
}
