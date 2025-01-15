/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */

use crate::util::error;
use crate::util::error::ErrorType;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;

pub fn check_entry<'a>(registry: &'a [Mod], hash: &str) -> error::Result<(usize, &'a Mod)> {
    match registry
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.hash == hash)
    {
        Some((index, entry)) => Ok((index, entry)),
        None => Err(VaultError::HashNotFound
            .builder()
            .context("Hash", hash)
            .build()),
    }
}

pub(crate) fn filter_registry<'a>(
    registry: &'a [Mod],
    hash: &Option<String>,
    id: &Option<String>,
) -> Vec<&'a Mod> {
    registry
        .iter()
        .filter(|&entry| {
            let match_hash = if let Some(hash) = hash {
                entry.hash.contains(hash)
            } else {
                true
            };

            let match_id = if let Some(id) = id {
                entry.meta.id.contains(id)
            } else {
                true
            };

            match_hash && match_id
        })
        .collect::<Vec<&Mod>>()
}
