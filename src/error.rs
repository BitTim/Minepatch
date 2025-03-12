/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       error.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   01.03.25, 00:53
 */
use crate::bundle::BundleError;
use crate::common::event::{Event, EventError};
use crate::common::file::error::FileError;
use crate::common::meta::error::MetaError;
use crate::instance::InstanceError;
use crate::patch::PatchError;
use crate::template::TemplateError;
use crate::vault::VaultError;
use std::sync::mpsc;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Something went wrong: {0}")]
    Generic(String),

    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    Meta(#[from] MetaError),
    #[error(transparent)]
    Event(#[from] EventError),
    #[error(transparent)]
    Vault(#[from] VaultError),
    #[error(transparent)]
    Template(#[from] TemplateError),
    #[error(transparent)]
    Patch(#[from] PatchError),
    #[error(transparent)]
    Bundle(#[from] BundleError),
    #[error(transparent)]
    Instance(#[from] InstanceError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SendEvent(#[from] mpsc::SendError<Event>),
    #[error(transparent)]
    SendBool(#[from] mpsc::SendError<bool>),
    #[error(transparent)]
    SendVecUsize(#[from] mpsc::SendError<Vec<usize>>),
    #[error(transparent)]
    Recv(#[from] mpsc::RecvError),
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
    #[error(transparent)]
    BinCode(#[from] bincode::Error),
}
