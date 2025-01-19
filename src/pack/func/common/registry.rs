/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       registry.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 13:55
 */
use crate::pack::data::Pack;
use crate::pack::error::PackError;
use crate::prelude::*;

pub fn check_pack<'a>(packs: &'a [Pack], name: &str) -> Result<(usize, &'a Pack)> {
    match packs
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.name == name)
    {
        Some((index, entry)) => Ok((index, entry)),
        None => Err(Error::Pack(PackError::PackNotFound(name.to_owned()))),
    }
}
