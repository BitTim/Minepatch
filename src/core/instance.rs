/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       instance.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 17:05
 */
use crate::core::file;
use std::{fs, io};

const INSTANCES_FILE_NAME: &str = "instances.csv";

pub fn list() -> io::Result<()> {
    let data_path = file::get_data_path().unwrap();
    let instances_file_path = data_path.join(&INSTANCES_FILE_NAME);

    let file = if fs::exists(&instances_file_path)? {
        fs::File::open(&instances_file_path)?
    } else {
        fs::File::create(&instances_file_path)?
    };

    let mut csv_reader = csv::Reader::from_reader(file);

    for result in csv_reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
