/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   16.01.25, 17:32
 */
use crate::pack::data::Pack;
use crate::pack::error::PackError;
use crate::util::error;
use crate::util::error::ErrorType;

pub fn check_pack<'a>(packs: &'a [Pack], name: &str) -> error::Result<(usize, &'a Pack)> {
    match packs
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.name == name)
    {
        Some((index, entry)) => Ok((index, entry)),
        None => Err(PackError::PackNotFound
            .builder()
            .context("Name", name)
            .build()),
    }
}
