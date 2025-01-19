/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:58
 */

use crate::prelude::*;
use crate::vault::data::Mod;
use crate::vault::error::VaultError;

pub fn check_entry<'a>(registry: &'a [Mod], hash: &str) -> Result<(usize, &'a Mod)> {
    match registry
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.hash == hash)
    {
        Some((index, entry)) => Ok((index, entry)),
        None => Err(Error::Vault(VaultError::HashNotFound(hash.to_owned()))),
    }
}

pub(crate) fn _filter_registry<'a>(
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
                match &entry.meta.id {
                    Some(entity_id) => entity_id.contains(id),
                    None => false,
                }
            } else {
                true
            };

            match_hash && match_id
        })
        .collect::<Vec<&Mod>>()
}
