/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       fabric_based.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   03.01.25, 23:58
 */
use crate::commands::vault::meta::Meta;
use crate::common::error;

pub(crate) fn extract_meta(data: &str, loader: &str) -> error::Result<Meta> {
    println!("{}\n{}", data, loader);
    todo!()
}
