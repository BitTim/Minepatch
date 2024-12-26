/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 18:36
 */
use crate::data::instance::{Instance, InstanceDisplay};
use crate::logic::file;
use csv::Reader;
use std::path::PathBuf;
use std::{fs, io};
use tabled::settings::object::{Columns, Rows};
use tabled::settings::width::MinWidth;
use tabled::settings::{Alignment, Format, Style};
use tabled::Table;

const INSTANCES_FILE_NAME: &str = "instances.csv";

fn init_file() -> io::Result<PathBuf> {
    let dir = file::get_data_path()?;
    fs::create_dir_all(&dir)?;

    let path = dir.join(&INSTANCES_FILE_NAME);
    if fs::exists(&path)? == false {
        fs::File::create(&path)?;
    }

    Ok(path)
}

fn read_instances() -> io::Result<Vec<Instance>> {
    let path = init_file()?;
    let mut csv_reader = Reader::from_path(&path)?;

    let mut instances: Vec<Instance> = vec![];
    for result in csv_reader.deserialize() {
        instances.push(result?);
    }

    Ok(instances)
}

pub fn list() -> io::Result<()> {
    let instances = read_instances()?;
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
