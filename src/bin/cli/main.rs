/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   20.01.25, 03:13
 */
use crate::cli::instance::InstanceCommands;
use crate::cli::pack::PackCommands;
use crate::cli::{vault, Cli, Commands};
use clap::Parser;
use cli::vault::VaultCommands;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use minepatch::prelude::*;
use minepatch::update::func;
use minepatch::{db, instance, pack};
use rusqlite::Connection;
use std::ffi::OsStr;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, Process, ProcessStatus, System};

mod cli;
mod output;

fn match_command(command: &Commands, connection: Connection) -> Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::List => {
                instance::func::list::list()?;
            }
            InstanceCommands::Link { path, name } => {
                instance::func::link::link(path, name)?;
            }
            InstanceCommands::Rename { name, new_name } => {
                instance::func::rename::rename(name, new_name)?;
            }
            InstanceCommands::Unlink { name, all, yes } => {
                instance::func::unlink::unlink(name, all, yes)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, overwrite } => {
                vault::add(&connection, path, overwrite)?;
            }
            VaultCommands::List {
                detailed,
                hash,
                id,
                name,
            } => {
                vault::list(&connection, detailed, hash, id, name)?;
            }
            VaultCommands::Remove { hash, all, yes } => {
                vault::remove(hash, *all, *yes)?;
            }
        },
        Commands::Pack { pack_command } => match pack_command {
            PackCommands::List => {}
            PackCommands::Create {
                name,
                from,
                instance,
                silent,
            } => {
                pack::create(name, from, instance, silent)?;
            }
            PackCommands::Delete => {}
        },
    }

    Ok(())
}

fn await_exclusive() -> Result<()> {
    let pid = process::id();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}")?);
    spinner.enable_steady_tick(Duration::from_millis(100));

    loop {
        let sys = System::new_all();
        let processes = sys
            .processes_by_name(OsStr::new("minepatch"))
            .collect::<Vec<&Process>>();

        match &processes.is_empty() {
            true => {
                break;
            }
            false => {
                let mut counter = 0;
                let total = processes.len() - 1;

                for proc in &processes {
                    if proc.pid() == Pid::from_u32(pid) {
                        continue;
                    }

                    if let ProcessStatus::Run = proc.status() {
                        counter += 1;

                        spinner.set_message(format!(
                            "{}\n\t{}{}\n\t{}{}\n\t{}{}s\n\t{}{}",
                            "Minepatch is already running. Waiting for other process to end to prevent data corruption.".bold().purple(),
                            "Name:     ".bold().yellow(),
                            proc.name().to_str().unwrap_or("").trim(),
                            "PID:      ".bold().yellow(),
                            proc.pid().to_string().trim(),
                            "Time:     ".bold().yellow(),
                            proc.run_time(),
                            "Conflict: ".bold().yellow(),
                            format!(
                                "{} / {}",
                                counter,
                                total
                            ).bold().cyan()
                        ));

                        sleep(Duration::from_millis(1000));
                    }
                }

                if counter > 0 {
                    continue;
                }
                break;
            }
        }
    }

    Ok(())
}

fn error_handled() -> Result<()> {
    await_exclusive()?;
    let connection = db::init()?;

    let cli = Cli::parse();
    match_command(&cli.command, connection)?;

    Ok(())
}

fn main() {
    if let Err(error) = error_handled() {
        println!("{}", error);
    }
}
