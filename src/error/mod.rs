/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 02:41
 */
pub mod instance;
pub mod path;

use colored::Colorize;
use std::error;
use std::fmt::{Debug, Display, Formatter};

pub trait ErrorType: Debug {
    fn message(&self) -> &str;
    fn hint(&self) -> &str;
}

#[derive(Debug)]
pub struct Error {
    error_type: Box<dyn ErrorType>,
    context: Option<String>,
}

impl Error {
    pub(crate) fn new(error_type: Box<dyn ErrorType>, context: Option<String>) -> Box<Self> {
        Box::new(Self {
            error_type,
            context,
        })
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n",
            "error: ".red().bold(),
            self.error_type.message()
        )?;
        if let Some(context) = &self.context {
            write!(f, "\t{}\n\n", context.yellow())?;
        }
        write!(f, "{}", self.error_type.hint())?;

        Ok(())
    }
}
