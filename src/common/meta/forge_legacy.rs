/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       forge_legacy.rs
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
        obj.get("authorList")?
            .as_array()?
            .iter()
            .filter_map(|author| author.as_str().map(|author| author.to_owned()))
            .collect(),
    )
}

fn meta_from_obj(obj: &Value, loader: &str, filename: &str) -> Option<Meta> {
    let mod_obj = obj.as_array()?.first()?;

    Some(Meta::new(
        extract_string_json(mod_obj, "modid"),
        extract_string_json(mod_obj, "name").unwrap_or(filename.to_owned()),
        extract_string_json(mod_obj, "version"),
        extract_string_json(mod_obj, "description"),
        extract_authors(mod_obj),
        Some(loader.to_owned()),
        None,
        extract_string_json(mod_obj, "mcversion"),
    ))
}
