/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       file_utils.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.01.25, 21:22
 */
use crate::error::Error;
use crate::file::error::FileError;
use crate::prelude::*;
use std::fs;
use std::path::Path;

pub fn check_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        Err(Error::File(FileError::PathNotFound(
            path.display().to_string(),
        )))
    } else {
        Ok(())
    }
}

pub fn move_file(path: &Path, new_path: &Path) -> Result<()> {
    fs::copy(path, new_path)?;
    fs::remove_file(path)?;
    Ok(())
}

pub fn remove_empty_dirs(path: &Path) -> Result<bool> {
    if path.is_dir() {
        let mut is_empty = true;
        for entry in fs::read_dir(path)?.flatten() {
            let sub_path = entry.path();
            if !remove_empty_dirs(&sub_path)? {
                is_empty = false;
            }
        }

        if is_empty {
            fs::remove_dir(path)?
        }
        return Ok(is_empty);
    }

    Ok(false)
}
