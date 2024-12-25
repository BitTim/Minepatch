/*
 * Copyright (c) 2024 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       args.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.12.24, 02:08
 */

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Testing
    Test,
}
