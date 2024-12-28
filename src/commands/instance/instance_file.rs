/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_file.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   28.12.24, 02:06
 */

use crate::commands::instance::Instance;
use crate::common::error;
use crate::common::file::get_data_path;
use crate::constants::INSTANCES_FILE_NAME;
use csv::{Reader, Writer};
use std::fs;
use std::path::PathBuf;

fn init_file() -> error::Result<PathBuf> {
    let dir = get_data_path()?;
    fs::create_dir_all(&dir)?;

    let path = dir.join(&INSTANCES_FILE_NAME);
    if fs::exists(&path)? == false {
        fs::File::create(&path)?;
    }

    Ok(path)
}

pub fn read_all() -> error::Result<Vec<Instance>> {
    let path = init_file()?;
    let mut reader = Reader::from_path(&path)?;

    let mut instances: Vec<Instance> = vec![];
    for result in reader.deserialize() {
        instances.push(result?);
    }

    Ok(instances)
}

pub fn write_all(instances: Vec<Instance>) -> error::Result<()> {
    let path = init_file()?;
    let mut writer = Writer::from_path(&path)?;

    instances
        .iter()
        .try_for_each(|instance| writer.serialize(instance))?;

    writer.flush()?;
    Ok(())
}
