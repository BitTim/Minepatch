/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       fabric.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 03:07
 */
use crate::common::meta::common::{extract_meta_json, extract_string_json};
use crate::common::meta::data::Meta;
use crate::prelude::*;
use serde_json::Value;

pub(crate) fn extract_meta(data: &str, loader: &str, filename: &str) -> Result<Meta> {
    extract_meta_json(data, loader, filename, meta_from_obj)
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

fn meta_from_obj(obj: &Value, loader: &str, filename: &str) -> Option<Meta> {
    let depends_obj = obj.get("depends");
    let (loader_version, minecraft_version) = if let Some(depends_obj) = depends_obj {
        (
            extract_string_json(depends_obj, "fabricloader"),
            extract_string_json(depends_obj, "minecraft"),
        )
    } else {
        (None, None)
    };

    Some(Meta::new(
        extract_string_json(obj, "id"),
        extract_string_json(obj, "name").unwrap_or(filename.to_owned()),
        extract_string_json(obj, "version"),
        extract_string_json(obj, "description"),
        extract_authors(obj),
        Some(loader.to_owned()),
        loader_version,
        minecraft_version,
    ))
}
