/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       forge_legacy.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 17:53
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
        obj.get("authorList")?
            .as_array()?
            .iter()
            .filter_map(|author| author.as_str().map(|author| author.to_owned()))
            .collect(),
    )
}

fn meta_from_obj(obj: &Value, loader: &str) -> Option<Meta> {
    let mod_obj = obj.as_array()?.first()?;

    Some(Meta {
        id: extract_string_json(mod_obj, "modid")?,
        name: extract_string_json(mod_obj, "name")?,
        version: extract_string_json(mod_obj, "version")?,
        description: extract_string_json(mod_obj, "description"),
        authors: extract_authors(mod_obj),
        loader: loader.to_owned(),
        loader_version: None,
        minecraft_version: extract_string_json(mod_obj, "mcversion"),
    })
}