/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       common.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 12:29
 */
use crate::common::meta::data::Meta;
use crate::common::meta::error::MetaError;
use crate::prelude::*;
use serde_json::Value;

pub(crate) fn extract_meta_json<F>(data: &str, loader: &str, meta_from_obj: F) -> Result<Meta>
where
    F: FnOnce(&Value, &str) -> Option<Meta>,
{
    meta_from_obj(&parse_json(data)?, loader)
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
