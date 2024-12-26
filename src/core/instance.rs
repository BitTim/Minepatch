/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   26.12.24, 02:58
 */
use crate::core::file;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tabled::builder::Builder;
use tabled::settings::Style;

const INSTANCES_FILE_NAME: &str = "instances.csv";

fn get_instance_file_path() -> io::Result<PathBuf> {
    let dir = file::get_data_path()?;
    fs::create_dir_all(&dir)?;

    Ok(dir.join(&INSTANCES_FILE_NAME))
}

fn _write_csv(file_path: &Path, records: Vec<Vec<String>>) -> io::Result<()> {
    let mut csv_writer = csv::Writer::from_path(&file_path)?;

    csv_writer.write_record(&["id", "path"])?;
    for record in records {
        csv_writer.write_record(record)?
    }

    csv_writer.flush()?;
    Ok(())
}

pub fn list() -> io::Result<()> {
    let file_path = get_instance_file_path()?;
    if fs::exists(&file_path)? == false {
        return Ok(());
    };

    let mut table_builder = Builder::default();
    let mut csv_reader = csv::Reader::from_path(&file_path)?;

    table_builder.push_record(csv_reader.headers()?);
    for record in csv_reader.records() {
        table_builder.push_record(&record?);
    }

    println!(
        "{}",
        table_builder.build().with(Style::rounded()).to_string()
    );
    Ok(())
}
