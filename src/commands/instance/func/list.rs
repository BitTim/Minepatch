/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       list.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   13.01.25, 21:41
 */
use crate::commands::instance::data::Instance;
use crate::commands::instance::output::InstanceDisplay;
use crate::common::output::table::TableOutput;
use crate::common::output::Output;
use crate::common::{error, file};
use tabled::settings::object::Columns;

pub fn list() -> error::Result<()> {
    let instances: Vec<Instance> = file::read_all()?;
    let instance_displays = instances
        .iter()
        .map(|instance| instance.to_display())
        .collect::<Vec<InstanceDisplay>>();

    TableOutput::new(instance_displays)
        .center(Columns::new(2..3))
        .print();
    Ok(())
}
