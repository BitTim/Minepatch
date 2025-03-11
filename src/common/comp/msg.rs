/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       msg.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.03.25, 07:28
 */

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum CompProcess {
    Serialize,
    Deserialize,
    Compress,
    Decompress,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum CompMessage {
    SerializeStatus,
}
