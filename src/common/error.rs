/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   28.12.24, 13:58
 */

use colored::Colorize;
use std::fmt::{Debug, Display, Formatter};
use std::{error, result};

pub trait ErrorType: Debug {
    fn message(&self) -> String;
    fn hint(&self) -> String;

    fn builder(self) -> Error
    where
        Self: Sized + 'static,
    {
        Error::new(Box::new(self))
    }
}

#[derive(Debug)]
pub struct Error {
    error_type: Box<dyn ErrorType>,
    contexts: Vec<String>,
}

impl Error {
    fn new(error_type: Box<dyn ErrorType>) -> Self {
        Self {
            error_type,
            contexts: vec![],
        }
    }

    pub(crate) fn context(mut self, context: &str) -> Self {
        self.contexts.push(context.to_owned());
        self
    }

    pub(crate) fn build(self) -> Box<Self> {
        Box::new(self)
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "error: ".red().bold(), self.error_type.message())?;

        for context in &self.contexts {
            write!(f, "\n\t{}", context.yellow())?;
        }

        if !self.error_type.hint().is_empty() {
            write!(f, "\n{}", self.error_type.hint())?;
        }

        Ok(())
    }
}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum CommonError {
    Unknown,
    Wrapper(Box<dyn error::Error>),
}

impl ErrorType for CommonError {
    fn message(&self) -> String {
        match self {
            CommonError::Unknown => "Something went wrong".to_owned(),
            CommonError::Wrapper(error) => error.to_string(),
        }
    }

    fn hint(&self) -> String {
        match self {
            CommonError::Unknown => "An unknown error occurred, we also don't know what happened",
            CommonError::Wrapper(_) => "",
        }
        .to_owned()
    }
}
