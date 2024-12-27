/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_logic.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 16:20
 */
use crate::commands::instance::instance_error::InstanceError;
use crate::commands::instance::{Instance, InstanceDisplay};
use crate::util::error;
use crate::util::file::error::FileError;
use crate::util::file::{get_data_path, get_filename};
use csv::{Reader, Writer};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tabled::settings::object::{Columns, Rows};
use tabled::settings::width::MinWidth;
use tabled::settings::{Alignment, Format, Style};
use tabled::Table;

const INSTANCES_FILE_NAME: &str = "instances.csv";

fn init_file() -> io::Result<PathBuf> {
    let dir = get_data_path()?;
    fs::create_dir_all(&dir)?;

    let path = dir.join(&INSTANCES_FILE_NAME);
    if fs::exists(&path)? == false {
        fs::File::create(&path)?;
    }

    Ok(path)
}

fn read_all() -> io::Result<Vec<Instance>> {
    let path = init_file()?;
    let mut reader = Reader::from_path(&path)?;

    let mut instances: Vec<Instance> = vec![];
    for result in reader.deserialize() {
        instances.push(result?);
    }

    Ok(instances)
}

fn write_all(instances: Vec<Instance>) -> io::Result<()> {
    let path = init_file()?;
    let mut writer = Writer::from_path(&path)?;

    instances
        .iter()
        .try_for_each(|instance| writer.serialize(instance))?;

    writer.flush()?;
    Ok(())
}

fn check_free(instances: &[Instance], name: &str) -> Result<(), Box<dyn Error>> {
    match instances.iter().any(|instance| instance.name == name) {
        true => Err(error::Error::new(
            Box::new(InstanceError::NameTaken),
            Some(format!("Name: '{}'", name)),
        )),
        false => Ok(()),
    }
}

fn check_exists(instances: &[Instance], name: &str) -> Result<(), Box<dyn Error>> {
    match instances.iter().any(|instance| instance.name == name) {
        true => Ok(()),
        false => Err(error::Error::new(
            Box::new(InstanceError::NameNotFound),
            Some(format!("Name: '{}'", name)),
        )),
    }
}

fn find_instance<'a>(
    instances: &'a [Instance],
    name: &str,
) -> Result<&'a Instance, Box<dyn Error>> {
    instances
        .iter()
        .find(|instance| instance.get_name() == name)
        .ok_or(error::Error::new(
            Box::new(InstanceError::NameTaken),
            Some(format!("Name: '{}'", name)),
        ))
}

fn find_instance_mut<'a>(
    instances: &'a mut [Instance],
    name: &str,
) -> Result<&'a mut Instance, Box<dyn Error>> {
    instances
        .iter_mut()
        .find(|instance| instance.get_name() == name)
        .ok_or(error::Error::new(
            Box::new(InstanceError::NameNotFound),
            Some(format!("Name: '{}'", name)),
        ))
}

pub fn list() -> io::Result<()> {
    let instances = read_all()?;
    let instance_displays = instances
        .iter()
        .map(|instance| instance.to_display())
        .collect::<Vec<InstanceDisplay>>();

    let table = Table::new(instance_displays)
        .with(Style::rounded().remove_horizontals())
        .modify(
            Rows::new(0..1),
            Format::content(|s| format!("\x1b[1m\x1b[4m{}\x1b[22m\x1b[24m", s)),
        )
        .modify(Columns::new(0..1), MinWidth::new(16))
        .modify(Columns::new(1..2), MinWidth::new(32))
        .modify(Columns::new(2..3), Alignment::center())
        .to_string();

    println!("{}", table);
    Ok(())
}

pub fn link(path: &Path, name: &Option<String>) -> Result<(), Box<dyn Error>> {
    if fs::exists(&path)? == false {
        return Err(error::Error::new(
            Box::new(FileError::PathNotFound),
            Some(format!("Path: '{}'", path.display())),
        ));
    }

    let actual_name = match name {
        Some(name) => name,
        None => get_filename(&path)?,
    };

    let mut instances = read_all()?;
    check_free(&instances, actual_name)?;

    instances.push(Instance::new(actual_name.to_string(), path.to_path_buf()));
    write_all(instances)?;
    Ok(())
}

pub fn rename(name: &str, new_name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let mut instances = read_all()?;
    check_exists(&instances, name)?;

    let actual_new_name = match new_name {
        Some(name) => name,
        None => {
            let instance = find_instance(&instances, name)?;
            get_filename(&instance.path)?
        }
    }
    .to_string();

    if name == actual_new_name {
        return Err(error::Error::new(
            Box::new(InstanceError::NameNotChanged),
            Some(format!("Name: '{}'", actual_new_name)),
        ));
    }

    check_free(&instances, &*actual_new_name)?;
    let instance = find_instance_mut(&mut instances, name)?;
    instance.set_name(actual_new_name);

    write_all(instances)?;
    Ok(())
}
