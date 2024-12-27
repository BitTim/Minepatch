/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       mod.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   27.12.24, 16:20
 */
use crate::commands::instance::instance_cli;
use clap::{Parser, Subcommand};

pub mod instance;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage relevant Minecraft instances
    Instance {
        #[command(subcommand)]
        instance_commands: instance_cli::InstanceCommands,
    },
}
