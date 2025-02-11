/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       event.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:51
 */
use crate::prelude::{Error, Message, Process};
use std::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    Progress {
        process: Process,
        total: Option<u64>,
    },
    ProgressTick {
        process: Process,
        message: Message,
    },
    ProgressFinish {
        process: Process,
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

    Success {
        message: Message,
    },

    Warning {
        warning: Box<Error>,
    },

    Log {
        message: Message,
    },
}
