/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       queries.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 01:26
 */

pub(crate) enum InstanceQuery {
    GeneralFilter,
    ByName,
}

impl InstanceQuery {
    pub(super) fn value(&self) -> String {
        match self {
            InstanceQuery::GeneralFilter => {
                "SELECT name, path, pack, patch FROM instance WHERE name LIKE ?1||'%'"
            }
            InstanceQuery::ByName => "SELECT name, path, pack, patch FROM instance WHERE name = ?1",
        }
        .to_owned()
    }
}
