/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_file.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 18:17
 */

use crate::commands::instance::Instance;
use crate::common::file::get_data_path;
use crate::constants::INSTANCES_FILE_NAME;
use csv::{Reader, Writer};
use std::path::PathBuf;
use std::{fs, io};

fn init_file() -> io::Result<PathBuf> {
    let dir = get_data_path()?;
    fs::create_dir_all(&dir)?;

    let path = dir.join(&INSTANCES_FILE_NAME);
    if fs::exists(&path)? == false {
        fs::File::create(&path)?;
    }

    Ok(path)
}

pub fn read_all() -> io::Result<Vec<Instance>> {
    let path = init_file()?;
    let mut reader = Reader::from_path(&path)?;

    let mut instances: Vec<Instance> = vec![];
    for result in reader.deserialize() {
        instances.push(result?);
    }

    Ok(instances)
}

pub fn write_all(instances: Vec<Instance>) -> io::Result<()> {
    let path = init_file()?;
    let mut writer = Writer::from_path(&path)?;

    instances
        .iter()
        .try_for_each(|instance| writer.serialize(instance))?;

    writer.flush()?;
    Ok(())
}
