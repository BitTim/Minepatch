/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   05.01.25, 19:21
 */

use crate::common::output::status::{State, StatusOutput};
use std::fmt::{Debug, Display, Formatter};
use std::{error, result};

pub trait ErrorType: Debug {
    fn message(&self) -> String;
    fn hint(&self) -> String;

    fn builder(self) -> Error
    where
        Self: Sized + 'static,
    {
        Error::from(Box::new(self))
    }
}

#[derive(Debug)]
pub struct Error {
    status_output: StatusOutput,
}

impl Error {
    fn from(error_type: Box<dyn ErrorType>) -> Self {
        Self {
            status_output: StatusOutput::new(State::Error, &*error_type.message())
                .hint(error_type.hint()),
        }
    }

    pub(crate) fn context(mut self, title: &str, content: &str) -> Self {
        self.status_output = self.status_output.context(title, content);
        self
    }

    pub(crate) fn build(self) -> Box<Self> {
        Box::new(self)
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_output)?;
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
