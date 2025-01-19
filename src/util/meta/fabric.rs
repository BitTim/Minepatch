/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       fabric.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 17:54
 */
use crate::util::error;
use crate::util::meta::common::{extract_meta_json, extract_string_json};
use crate::util::meta::data::Meta;
use serde_json::Value;

pub(crate) fn extract_meta(data: &str, loader: &str) -> error::Result<Meta> {
    extract_meta_json(data, loader, meta_from_obj)
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
    let depends_obj = obj.get("depends");
    let (loader_version, minecraft_version) = if let Some(depends_obj) = depends_obj {
        (
            extract_string_json(depends_obj, "fabricloader"),
            extract_string_json(depends_obj, "minecraft"),
        )
    } else {
        (None, None)
    };

    Some(Meta {
        id: extract_string_json(obj, "id")?,
        name: extract_string_json(obj, "name")?,
        version: extract_string_json(obj, "version")?,
        description: extract_string_json(obj, "description"),
        authors: extract_authors(obj),
        loader: loader.to_owned(),
        loader_version,
        minecraft_version,
    })
}
