/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       common.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 17:50
 */
use crate::util::error;
use crate::util::error::{Error, ErrorType};
use crate::util::meta::data::Meta;
use crate::util::meta::error::MetaError;
use serde_json::Value;

pub(crate) fn extract_meta_json<F>(
    data: &str,
    loader: &str,
    meta_from_obj: F,
) -> error::Result<Meta>
where
    F: FnOnce(&Value, &str) -> Option<Meta>,
{
    meta_from_obj(&parse_json(data)?, loader).ok_or(build_malformed_error(loader))
}

pub(crate) fn parse_json(data: &str) -> error::Result<Value> {
    let mut trimmed_data = data.to_owned();
    trimmed_data.retain(|c| c != '\n');

    Ok(serde_json::from_str(&trimmed_data)?)
}

pub(crate) fn build_malformed_error(loader: &str) -> Box<Error> {
    MetaError::MalformedMetaFile
        .builder()
        .context("Detected loader", loader)
        .build()
}

pub(crate) fn extract_string_json(obj: &Value, key: &str) -> Option<String> {
    Some(obj.get(key)?.as_str()?.trim().to_owned())
}
