/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   12.02.25, 04:16
 */
use crate::cli::instance::InstanceCommands;
use crate::cli::pack::PackCommands;
use crate::cli::patch::PatchCommands;
use crate::cli::template::TemplateCommands;
use crate::cli::{instance, pack, patch, template, vault, Cli, Commands};
use crate::output::status::{Status, StatusOutput};
use crate::output::Output;
use clap::Parser;
use cli::update::func;
use cli::vault::VaultCommands;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use minepatch::db;
use minepatch::hash::{HashMessage, HashProcess};
use minepatch::instance::{InstanceMessage, InstanceProcess};
use minepatch::msg::Process;
use minepatch::pack::{PackMessage, PackProcess};
use minepatch::patch::{PatchMessage, PatchProcess};
use minepatch::prelude::*;
use minepatch::template::{TemplateMessage, TemplateProcess};
use minepatch::vault::{ModMessage, ModProcess};
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

mod cli;
mod output;

fn match_command(command: &Commands, connection: &Connection, tx: &Sender<Event>) -> Result<()> {
    match command {
        Commands::Update => func::update::update()?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::Apply { name, patch } => {
                instance::apply(connection, tx, name, patch)?
            }
            InstanceCommands::List { name } => instance::list(connection, tx, name)?,
            InstanceCommands::Link { path, name, pack } => {
                instance::link(connection, tx, path, name, pack)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, overwrite } => {
                vault::add(connection, tx, path, overwrite)?;
            }
            VaultCommands::List {
                detailed,
                hash,
                id,
                name,
            } => {
                vault::list(connection, tx, detailed, hash, id, name)?;
            }
            VaultCommands::Remove { hash, all, yes } => {
                vault::remove(connection, tx, hash, *all, *yes)?;
            }
        },
        Commands::Template {
            template_commands: template_command,
        } => match template_command {
            TemplateCommands::Create {
                name,
                version,
                loader,
                download,
            } => template::create(connection, tx, name, version, loader, download)?,
            TemplateCommands::List { name } => template::list(connection, name)?,
        },
        Commands::Patch {
            patch_commands: patch_command,
        } => match patch_command {
            PatchCommands::Create {
                name,
                dependency,
                pack,
            } => patch::create(connection, tx, name, pack, dependency)?,
            PatchCommands::Exclude {
                name,
                pack,
                mod_hash,
            } => patch::exclude(connection, tx, name, pack, mod_hash)?,
            PatchCommands::Generate { name, instance } => {
                patch::generate(connection, tx, name, instance)?
            }
            PatchCommands::Include {
                name,
                pack,
                mod_hash,
            } => patch::include(connection, tx, name, pack, mod_hash)?,
            PatchCommands::List { name, pack } => patch::list(connection, tx, name, pack)?,
            PatchCommands::Simulate {
                name,
                pack,
                dir_hash,
            } => patch::simulate(connection, tx, name, pack, dir_hash)?,
            PatchCommands::View { name, pack } => patch::view(connection, tx, name, pack)?,
        },
        Commands::Pack {
            pack_commands: pack_command,
        } => match pack_command {
            PackCommands::List { name } => pack::list(connection, tx, name)?,
            PackCommands::Create {
                name,
                description,
                template,
                from,
                instance,
            } => pack::create(connection, tx, name, description, template, from, instance)?,
            PackCommands::Delete => {}
        },
    }

    Ok(())
}

fn match_process(process: &Process) -> String {
    match process {
        Process::Hash(process) => match process {
            HashProcess::HashFiles => "Hash mod files",
        },
        Process::Instance(process) => match process {
            InstanceProcess::Detect => "Detect patch and pack",
            InstanceProcess::Link => "Link instance",
            InstanceProcess::Apply => {}
            InstanceProcess::Validate => {}
        },
        Process::Pack(process) => match process {
            PackProcess::Create => "Create pack",
            PackProcess::AddModFiles => "Add mod files",
            PackProcess::Validate => {}
        },
        Process::Patch(process) => match process {
            PatchProcess::Simulate => "Simulate patch",
            PatchProcess::Create => {}
            PatchProcess::Exclude => {}
            PatchProcess::Generate => {}
            PatchProcess::HashModFiles => {}
            PatchProcess::Include => {}
            PatchProcess::Validate => {}
        },
        Process::Mod(process) => match process {
            ModProcess::Add => {}
            ModProcess::Remove => {}
            ModProcess::Validate => {}
        },
        Process::Template(process) => match process {
            TemplateProcess::Create => {}
            TemplateProcess::Validate => {}
        },
    }
    .to_owned()
}

fn match_message(message: &Message) -> String {
    match message {
        Message::Hash(message) => match message {
            HashMessage::HashFilesStatus { path } => {
                format!("Mod file path: '{}'", path.display().to_string().cyan())
            }
        },
        Message::Instance(message) => match message {
            InstanceMessage::LinkSuccess { instance } => format!(
                "Linked '{}' with pack '{}' and patch '{}'",
                instance.name.cyan(),
                instance.pack.cyan(),
                instance.patch.cyan()
            ),
            InstanceMessage::DetectSuccess { pack, patch } => {
                format!(
                    "Detected pack '{}' and patch '{}'",
                    pack.cyan(),
                    patch.cyan()
                )
            }
            InstanceMessage::ApplySuccess { .. } => {}
            InstanceMessage::ValidateSuccess { .. } => {}
        },
        Message::Pack(message) => match message {
            PackMessage::AddModFileStatus { path, hash } => {
                format!(
                    "Mod file path: '{}' ['{}']",
                    path.display().to_string().cyan(),
                    hash.yellow()
                )
            }
            PackMessage::CreateSuccess { pack } => format!("Created pack '{}'", name.cyan()),
            PackMessage::ValidateSuccess { .. } => {}
        },
        Message::Patch(message) => match message {
            PatchMessage::SimulateStatus { name } => format!("Patch: '{}'", name.cyan()),
            PatchMessage::CreateSuccess { .. } => {}
            PatchMessage::ExcludeSuccess { .. } => {}
            PatchMessage::GenerateSuccess { .. } => {}
            PatchMessage::HashModFileStatus { .. } => {}
            PatchMessage::IncludeSuccess { .. } => {}
            PatchMessage::ValidateSuccess { .. } => {}
        },
        Message::Mod(message) => match message {
            ModMessage::AddSuccess { .. } => {}
            ModMessage::RemoveStatus { .. } => {}
            ModMessage::ValidateSuccess { .. } => {}
        },
        Message::Template(message) => match message {
            TemplateMessage::CreateSuccess { .. } => {}
            TemplateMessage::ValidateSuccess { .. } => {}
        },
    }
}

fn match_event(rx: Receiver<Event>, processes: &mut HashMap<Process, ProgressBar>) -> Result<()> {
    let multi_progress = MultiProgress::new();

    for event in rx {
        match event {
            Event::Progress { process, total } => {
                let progress_bar = processes.remove(&process);
                if progress_bar.is_some() {
                    let progress_bar: ProgressBar = progress_bar.unwrap();
                    progress_bar.finish_and_clear();
                }

                let progress_bar = match total {
                    Some(total) => {
                        let progress_bar = ProgressBar::new(total);
                        progress_bar.set_style(
                            ProgressStyle::with_template(&format!(
                                "{{spinner}} [{}]: {{msg:!}}\n{{wide_bar}} {{percent:>3}} % ({{human_pos:>5}} / {{human_len:5}})\nElapsed: {{elapsed_precise}}\tETA: {{eta_precise}}",
                                match_process(&process).bold()
                            ))?.tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
                        );
                        progress_bar
                    }
                    None => {
                        let progress_bar = ProgressBar::new_spinner();
                        progress_bar.set_style(
                            ProgressStyle::with_template(&format!(
                                "{{spinner}} [{}]: {{msg:!}}",
                                match_process(&process).bold()
                            ))?
                            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
                        );
                        progress_bar
                    }
                };
                progress_bar.enable_steady_tick(Duration::from_millis(100));

                let progress_bar = multi_progress.add(progress_bar);
                processes.insert(process, progress_bar);
            }
            Event::ProgressTick { process, message } => {
                let progress_bar = processes.get(&process).unwrap();

                progress_bar.set_message(match_message(&message));
                progress_bar.inc(1);
            }
            Event::ProgressFinish { process, message } => {
                let progress_bar = processes.get(&process).unwrap();
                progress_bar.set_style(ProgressStyle::with_template(&format!(
                    "{} [{}]: {{msg}}",
                    "✓".bold().green(),
                    match_process(&process).bold().green()
                ))?);

                match message {
                    None => progress_bar.finish_and_clear(),
                    Some(message) => progress_bar.finish_with_message(match_message(&message)),
                }
            }
            Event::Confirm { tx, message } => {
                println!("[Confirm] Not yet implemented: {:?}, {:?}", tx, message)
            }
            Event::Select { tx, options } => {
                println!("[Select] Not yet implemented: {:?}, {:?}", tx, options)
            }
            Event::Warning { warning } => {
                multi_progress
                    .println(StatusOutput::new(Status::Warning, warning.to_string()).to_string())?;
            }
            Event::Log { message } => {
                multi_progress.println(match_message(&message))?;
            }
        }
    }

    Ok(())
}

fn fallible(rx: Receiver<Event>, thread: JoinHandle<Result<()>>) -> Result<()> {
    let mut processes = HashMap::new();

    match_event(rx, &mut processes)?;
    let result = thread.join().unwrap();

    if result.is_err() {
        for (process, progress_bar) in processes {
            progress_bar.set_style(ProgressStyle::with_template(&format!(
                "{} [{}]",
                "✗".bold().red(),
                match_process(&process).bold().red()
            ))?);
            progress_bar.finish_with_message("");
        }
    }

    result
}

fn main() {
    let (tx, rx) = mpsc::channel::<Event>();
    let cli = Cli::parse();

    let thread = thread::spawn(move || {
        let mut connection = db::init()?;
        let transaction = connection.transaction()?;
        let result = match_command(&cli.command, &transaction, &tx);
        if result.is_ok() {
            transaction.commit()?
        };

        result
    });

    if let Err(error) = fallible(rx, thread) {
        StatusOutput::new(Status::Error, error.to_string()).print();
    }
}
