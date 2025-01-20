/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 16:42
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Name '{0}' is already used by a template.")]
    NameExists(String),
}
