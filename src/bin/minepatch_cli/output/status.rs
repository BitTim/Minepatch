/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       status.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   14.02.25, 20:55
 */
use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Status {
    Error,
    Warning,
}

impl Status {
    fn title(&self) -> ColoredString {
        match self {
            Status::Error => "error: ".red(),
            Status::Warning => "warning: ".yellow(),
        }
        .bold()
    }
}

#[derive(Debug)]
pub struct StatusOutput {
    status: Status,
    message: String,
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.status.title(), self.message)
    }
}

impl StatusOutput {
    pub fn new(status: Status, message: String) -> Self {
        Self { status, message }
    }
}
