/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       status.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   11.02.25, 03:33
 */
use crate::output::Output;
use colored::{ColoredString, Colorize};
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
    message: String,
    context: Vec<(String, String)>,
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.status.title(), self.message)?;

        for (title, content) in &self.context {
            write!(
                f,
                "{}",
                self.status.colorize(&format!("\n\t{}: {}", title, content))
            )?;
        }

        Ok(())
    }
}

impl StatusOutput {
    pub fn new(status: Status, message: String) -> Self {
        Self {
            status,
            message,
            context: vec![],
        }
    }

    pub fn context(&mut self, title: String, content: String) -> &mut Self {
        self.context.push((title, content));
        self
    }
}

impl Output for StatusOutput {}
