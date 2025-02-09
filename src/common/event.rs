/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       event.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   09.02.25, 18:27
 */
use crate::msg::Message;
use crate::prelude::Error;
use std::sync::mpsc;
use uuid::Uuid;

#[derive(Debug)]
pub enum Event {
    Progress {
        id: Uuid,
        title: String,
        total: Option<u64>,
    },
    ProgressTick {
        id: Uuid,
        message: Message,
    },
    ProgressFinish {
        id: Uuid,
    },

    Confirm {
        tx: mpsc::Sender<bool>,
        message: Message,
    },

    // TODO: Adjust
    Select {
        tx: mpsc::Sender<String>,
        options: Vec<String>,
    },

    Warning {
        warning: Box<Error>,
    },

    Log {
        message: Message,
    },
}
