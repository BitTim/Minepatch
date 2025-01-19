/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       status.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   19.01.25, 14:01
 */
use crate::output::_Output;
use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum State {
    _Error,
    _Abort,
    _Success,
}

impl State {
    fn title(&self) -> ColoredString {
        match self {
            State::_Error => "error: ".red(),
            State::_Abort => "abort: ".yellow(),
            State::_Success => "success: ".green(),
        }
        .bold()
    }

    fn colorize(&self, message: &str) -> ColoredString {
        match self {
            State::_Error => message.yellow(),
            State::_Abort => message.green(),
            State::_Success => message.cyan(),
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
    pub fn _new(state: State, message: &str) -> Self {
        Self {
            state,
            message: message.to_owned(),
            contexts: vec![],
            hint: String::new(),
        }
    }

    pub fn _context(mut self, title: &str, content: &str) -> Self {
        self.contexts.push(Context {
            title: title.to_owned(),
            content: content.to_owned(),
        });
        self
    }

    pub fn _hint(mut self, hint: String) -> Self {
        self.hint = hint;
        self
    }
}

impl _Output for StatusOutput {}
