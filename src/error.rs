/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.02.25, 19:32
 */
use crate::common::file::error::FileError;
use crate::common::meta::error::MetaError;
use crate::instance::InstanceError;
use crate::pack::PackError;
use crate::patch::PatchError;
use crate::template::TemplateError;
use crate::vault::VaultError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Something went wrong: {0}")]
    Generic(String),

    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    Meta(#[from] MetaError),
    #[error(transparent)]
    Vault(#[from] VaultError),
    #[error(transparent)]
    Template(#[from] TemplateError),
    #[error(transparent)]
    Patch(#[from] PatchError),
    #[error(transparent)]
    Pack(#[from] PackError),
    #[error(transparent)]
    Instance(#[from] InstanceError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SQLite(#[from] rusqlite::Error),
    #[error(transparent)]
    JSON(#[from] serde_json::Error),
    #[error(transparent)]
    TOML(#[from] toml::de::Error),
    #[error(transparent)]
    Inquire(#[from] inquire::InquireError),
    #[error(transparent)]
    IndicatifTemplate(#[from] indicatif::style::TemplateError),
    #[error(transparent)]
    SelfUpdate(#[from] self_update::errors::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    RONSer(#[from] ron::error::Error),
    #[error(transparent)]
    RONDe(#[from] ron::error::SpannedError),
}
