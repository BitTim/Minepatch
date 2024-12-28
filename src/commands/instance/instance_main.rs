/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance_main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   28.12.24, 02:06
 */
use crate::commands::instance::instance_error::InstanceError;
use crate::commands::instance::{instance_file, instance_util, Instance, InstanceDisplay};
use crate::common::error;
use crate::common::error::ErrorType;
use crate::common::file::error::FileError;
use crate::common::file::get_filename;
use colored::Colorize;
use std::fs;
use std::path::Path;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::width::MinWidth;
use tabled::settings::{Alignment, Format, Style};
use tabled::Table;

pub fn list() -> error::Result<()> {
    let instances = instance_file::read_all()?;
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

pub fn link(path: &Path, name: &Option<String>) -> error::Result<()> {
    if fs::exists(&path)? == false {
        return Err(FileError::PathNotFound
            .builder()
            .context(&format!("Path: '{}'", path.display()))
            .build());
    }

    let actual_name = match name {
        Some(name) => name,
        None => get_filename(&path)?,
    };

    let mut instances = instance_file::read_all()?;
    instance_util::check_instance(&instances, actual_name, false)?;

    instances.push(Instance::new(actual_name, path));
    instance_file::write_all(instances)?;

    println!(
        "{}Linked instance\n\t{}\n\t{}",
        "success: ".green().bold(),
        format!("Name: '{}'", actual_name).cyan(),
        format!("Path: '{}'", path.display()).cyan()
    );
    Ok(())
}

pub fn rename(name: &str, new_name: &Option<String>) -> error::Result<()> {
    let mut instances = instance_file::read_all()?;
    let instance = instance_util::check_instance(&instances, name, true)?
        .unwrap()
        .1;

    let actual_new_name = match new_name {
        Some(name) => name,
        None => &get_filename(&instance.path)?.to_string(),
    };

    if name == actual_new_name {
        return Err(InstanceError::NameNotChanged
            .builder()
            .context(&format!("Name: '{}'", name))
            .context(&format!("New Name '{}", actual_new_name))
            .build());
    }

    instance_util::check_instance(&instances, &actual_new_name, false)?;
    let instance = instance_util::find_instance_mut(&mut instances, name)?;
    instance.set_name(actual_new_name);

    instance_file::write_all(instances)?;

    println!(
        "{}Renamed instance\n\t{}\n\t{}",
        "success: ".green().bold(),
        format!("Old name: '{}'", name).cyan(),
        format!("New name: '{}'", actual_new_name).cyan()
    );
    Ok(())
}

pub fn unlink(name: &Option<String>, all: &bool, yes: &bool) -> error::Result<()> {
    let mut instances = instance_file::read_all()?;

    let names: Vec<String> = if *all {
        instances
            .iter()
            .map(|instance| instance.name.to_string())
            .collect()
    } else {
        match name {
            Some(name) => vec![name.to_string()],
            None => return Err(InstanceError::NameNotFound.builder().build()),
        }
    };

    for name in names {
        let (index, instance) = instance_util::check_instance(&instances, &name, true)?.unwrap();
        if !yes && !instance_util::confirm_unlink(instance)? {
            continue;
        }
        instances.remove(index);

        println!(
            "{}Unlinked instance\n\t{}",
            "success: ".green().bold(),
            format!("Name: '{}'", name).cyan(),
        );
    }

    instance_file::write_all(instances)?;
    Ok(())
}
