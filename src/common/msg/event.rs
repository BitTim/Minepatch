/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       event.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.02.25, 00:00
 */
use crate::prelude::*;
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
        message: Option<Message>,
    },

    Confirm {
        tx: mpsc::Sender<bool>,
        message: Message,
    },

    // TODO: Adjust
    Select {
        tx: mpsc::Sender<String>,
        message: Message,
        options: Vec<String>,
        multiselect: bool,
    },

    Warning {
        warning: Box<Error>,
    },

    Log {
        message: Message,
    },
}
