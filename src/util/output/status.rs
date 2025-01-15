/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       status.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:24
 */
use crate::util::output::Output;
use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum State {
    Error,
    Abort,
    Success,
}

impl State {
    fn title(&self) -> ColoredString {
        match self {
            State::Error => "error: ".red(),
            State::Abort => "abort: ".yellow(),
            State::Success => "success: ".green(),
        }
        .bold()
    }

    fn colorize(&self, message: &str) -> ColoredString {
        match self {
            State::Error => message.yellow(),
            State::Abort => message.green(),
            State::Success => message.cyan(),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    title: String,
    content: String,
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{}'", self.title, self.content)
    }
}

#[derive(Debug)]
pub struct StatusOutput {
    state: State,
    message: String,
    contexts: Vec<Context>,
    hint: String,
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.state.title(), self.message)?;

        for context in &self.contexts {
            write!(f, "\n\t{}", self.state.colorize(&format!("{}", context)))?;
        }

        if !&self.hint.is_empty() {
            write!(f, "\n{}", self.hint)?;
        }

        Ok(())
    }
}

impl StatusOutput {
    pub(crate) fn new(state: State, message: &str) -> Self {
        Self {
            state,
            message: message.to_owned(),
            contexts: vec![],
            hint: String::new(),
        }
    }

    pub(crate) fn context(mut self, title: &str, content: &str) -> Self {
        self.contexts.push(Context {
            title: title.to_owned(),
            content: content.to_owned(),
        });
        self
    }

    pub(crate) fn hint(mut self, hint: String) -> Self {
        self.hint = hint;
        self
    }
}

impl Output for StatusOutput {}
