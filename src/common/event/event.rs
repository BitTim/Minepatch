/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       event.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.03.25, 06:43
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
        increment: u64,
    },
    ProgressFinish {
        process: Process,
        message: Option<Message>,
    },

    Confirm {
        tx: mpsc::Sender<bool>,
        message: Message,
    },

    Select {
        tx: mpsc::Sender<Vec<usize>>,
        message: Message,
        options: Vec<Message>,
        multiselect: bool,
    },

    Warning {
        warning: Box<Error>,
    },

    Log {
        message: Message,
    },
}
