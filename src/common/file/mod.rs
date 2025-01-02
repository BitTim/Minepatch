/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   02.01.25, 22:25
 */

use crate::common;
use crate::common::data::DataType;
use crate::common::error::ErrorType;
use crate::common::file::error::FileError;
use csv::{Reader, Writer};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub mod error;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "BitTim";

const DATA_PATH_ERROR: &str = "Did not find the projects OS specific data folder";

pub(crate) fn get_data_path() -> io::Result<PathBuf> {
    match ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME")) {
        None => Err(io::Error::new(io::ErrorKind::NotFound, DATA_PATH_ERROR)),
        Some(project_dir) => {
            let dir = project_dir.data_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }
    }
}

pub(crate) fn init_data_file(filename: &str) -> common::error::Result<PathBuf> {
    let dir = get_data_path()?;
    let path = dir.join(filename);

    if fs::exists(&path)? == false {
        fs::File::create(&path)?;
    }

    Ok(path)
}

pub(crate) fn filename_from_path(path: &Path) -> Result<&str, Box<dyn std::error::Error>> {
    let context = format!("Path: '{}'", path.to_path_buf().display().to_string());

    Ok(path
        .file_name()
        .ok_or(FileError::PathNoFileName.builder().context(&context))?
        .to_str()
        .ok_or(FileError::PathInvalidUTF8.builder().context(&context))?)
}

pub fn read_all<T: DataType>() -> common::error::Result<Vec<T>> {
    let path = init_data_file(T::FILENAME)?;
    let mut reader = Reader::from_path(&path)?;

    let mut instances: Vec<T> = vec![];
    for result in reader.deserialize() {
        instances.push(result?);
    }

    Ok(instances)
}

pub fn write_all<T: DataType>(data: Vec<T>) -> common::error::Result<()> {
    let path = init_data_file(T::FILENAME)?;
    let mut writer = Writer::from_path(&path)?;

    data.iter().try_for_each(|entry| writer.serialize(entry))?;

    writer.flush()?;
    Ok(())
}
