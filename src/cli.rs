/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       cli.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 02:43
 */

pub mod instance;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage relevant Minecraft instances
    Instance {
        #[command(subcommand)]
        instance_commands: instance::Commands,
    },
}
