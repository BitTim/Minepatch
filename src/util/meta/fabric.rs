/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       fabric.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */
use crate::util::error;
use crate::util::error::ErrorType;
use crate::util::meta::data::Meta;
use crate::util::meta::error::MetaError;
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
    Some(obj.get(key)?.as_str()?.trim().to_owned())
}

fn extract_authors(obj: &Value) -> Option<Vec<String>> {
    Some(
        obj.get("authors")?
            .as_array()?
            .iter()
            .filter_map(|author| {
                author.as_object().and_then(|author| {
                    author
                        .get("name")
                        .and_then(|author| author.as_str().map(|author| author.to_owned()))
                })
            })
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
        authors: extract_authors(obj)?,
        loader: loader.to_owned(),
        loader_version: extract_string(obj.get("depends")?, "fabricloader"),
        minecraft_version: extract_string(obj.get("depends")?, "minecraft"),
    })
}
