/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       status.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 22:20
 */
use crate::output::Output;
use colored::{ColoredString, Colorize};
use minepatch::msg::Message;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Status {
    Error,
    Warning,
    Success,
}

impl Status {
    fn title(&self) -> ColoredString {
        match self {
            Status::Error => "error: ".red(),
            Status::Warning => "warning: ".yellow(),
            Status::Success => "success: ".green(),
        }
        .bold()
    }

    fn colorize(&self, message: &str) -> ColoredString {
        match self {
            Status::Error => message.yellow(),
            Status::Warning => message.green(),
            Status::Success => message.cyan(),
        }
    }
}

#[derive(Debug)]
pub struct StatusOutput {
    status: Status,
    message: Message,
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.status.title(), self.message.msg)?;

        for context in &self.message.context {
            write!(
                f,
                "{}",
                self.status
                    .colorize(&format!("\n\t{}: {}", context.title, context.content))
            )?;
        }

        Ok(())
    }
}

impl StatusOutput {
    pub fn new(status: Status, message: Message) -> Self {
        Self { status, message }
    }
}

impl Output for StatusOutput {}
