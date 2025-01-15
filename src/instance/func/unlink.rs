/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       unlink.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   15.01.25, 11:46
 */
use crate::instance::data::Instance;
use crate::instance::error::InstanceError;
use crate::instance::func::common::confirm::confirm_unlink;
use crate::instance::func::common::registry::check_instance;
use crate::util::error::ErrorType;
use crate::util::output::status::{State, StatusOutput};
use crate::util::output::Output;
use crate::util::{error, file};

pub fn unlink(name: &Option<String>, all: &bool, yes: &bool) -> error::Result<()> {
    let mut instances = file::read_all()?;

    let names: Vec<String> = if *all {
        instances
            .iter()
            .map(|instance: &Instance| instance.name.to_owned())
            .collect()
    } else {
        match name {
            Some(name) => vec![name.to_owned()],
            None => return Err(InstanceError::NameNotFound.builder().build()),
        }
    };

    for name in names {
        let (index, instance) = check_instance(&instances, &name, true)?.unwrap();
        if !yes && !confirm_unlink(instance)? {
            StatusOutput::new(State::Abort, "Did not unlink instance")
                .context("Name", &instance.name)
                .print();
            continue;
        }

        instances.remove(index);
        StatusOutput::new(State::Success, "Unlinked instance")
            .context("Name", &*name)
            .print();
    }

    file::write_all(instances)?;
    Ok(())
}
