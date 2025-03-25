/*
 * Copyright (c) 2024-2025 Tim Anhalt (BitTim)
 *
 * Project:    Minepatch
 * License:    GPLv3
 *
 * File:       main.rs
 * Author:     Tim Anhalt (BitTim)
 * Modified:   25.03.25, 18:24
 */
use crate::cli::bundle::BundleCommands;
use crate::cli::instance::InstanceCommands;
use crate::cli::patch::PatchCommands;
use crate::cli::{Cli, Commands, bundle, instance, patch, vault};
use crate::output::format_string_option;
use crate::output::status::{Status, StatusOutput};
use clap::Parser;
use cli::update::func;
use cli::vault::VaultCommands;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use inquire::{Confirm, Select};
use mpcore::bundle::{BundleMessage, BundleProcess};
use mpcore::comp::CompProcess;
use mpcore::db;
use mpcore::event::EventError;
use mpcore::hash::{HashMessage, HashProcess};
use mpcore::instance::{InstanceMessage, InstanceProcess};
use mpcore::msg::Process;
use mpcore::patch::{PatchMessage, PatchProcess};
use mpcore::prelude::*;
use mpcore::vault::{ModMessage, ModProcess};
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

mod cli;
mod output;

fn match_command(command: &Commands, conn: &Connection, tx: &Sender<Event>) -> Result<()> {
    match command {
        Commands::Update => func::update::update(tx)?,
        Commands::Instance {
            instance_commands: instance_command,
        } => match instance_command {
            InstanceCommands::Apply { name, patch } => instance::apply(conn, tx, name, patch)?,
            InstanceCommands::List { name } => instance::list(conn, tx, name)?,
            InstanceCommands::Link { path, name, bundle } => {
                instance::link(conn, tx, path, name, bundle)?;
            }
        },
        Commands::Vault {
            vault_commands: vault_command,
        } => match vault_command {
            VaultCommands::Add { path, overwrite } => {
                vault::add(conn, tx, path, overwrite)?;
            }
            VaultCommands::List {
                detailed,
                hash,
                id,
                name,
            } => {
                vault::list(conn, tx, detailed, hash, id, name)?;
            }
            VaultCommands::Remove { hash, all, yes } => {
                vault::remove(conn, tx, hash, *all, *yes)?;
            }
            VaultCommands::Clean => vault::clean(conn, tx)?,
        },
        Commands::Patch {
            patch_commands: patch_command,
        } => match patch_command {
            PatchCommands::Create {
                name,
                dependency,
                bundle,
            } => patch::create(conn, tx, name, bundle, dependency)?,
            PatchCommands::Exclude {
                name,
                bundle,
                mod_hash,
            } => patch::exclude(conn, tx, name, bundle, mod_hash)?,
            PatchCommands::Generate { name, instance } => {
                patch::generate(conn, tx, name, instance)?
            }
            PatchCommands::Include {
                name,
                bundle,
                mod_hash,
            } => patch::include(conn, tx, name, bundle, mod_hash)?,
            PatchCommands::List { name, bundle } => patch::list(conn, tx, name, bundle)?,
            PatchCommands::Simulate {
                name,
                bundle,
                dir_hash,
            } => patch::simulate(conn, tx, name, bundle, dir_hash)?,
            PatchCommands::View { name, bundle } => patch::view(conn, tx, name, bundle)?,
            PatchCommands::Delete { name, bundle } => patch::delete(conn, tx, name, bundle)?,
            PatchCommands::Rename {
                name,
                bundle,
                new_name,
            } => patch::rename(conn, tx, name, bundle, new_name)?,
        },
        Commands::Bundle {
            bundle_commands: bundle_command,
        } => match bundle_command {
            BundleCommands::List { name } => bundle::list(conn, tx, name)?,
            BundleCommands::Create {
                name,
                description,
                from,
                instance,
            } => bundle::create(conn, tx, name, description, from.as_deref(), instance)?,
            BundleCommands::Delete => {}
            BundleCommands::Export { name, path } => {
                bundle::export(conn, tx, name, path.as_deref())?
            }
            BundleCommands::Import { path, name } => {
                bundle::import(conn, tx, path, name.as_deref())?
            }
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
            InstanceProcess::Detect => "Detect patch and bundle",
            InstanceProcess::Link => "Link instance",
            InstanceProcess::Apply => "Apply patch",
            InstanceProcess::Validate => "Validate instance",
        },
        Process::Bundle(process) => match process {
            BundleProcess::Create => "Create bundle",
            BundleProcess::AddModFiles => "Add mod files",
            BundleProcess::Validate => "Validate bundle",
            BundleProcess::Export => "Export bundle",
            BundleProcess::Import => "Import bundle",
        },
        Process::Patch(process) => match process {
            PatchProcess::Simulate => "Simulate patch",
            PatchProcess::Create => "Create patch",
            PatchProcess::Exclude => "Exclude mod with patch",
            PatchProcess::Generate => "Generate patch from instance",
            PatchProcess::HashModFiles => "Hash mod files",
            PatchProcess::Include => "Include mod with patch",
            PatchProcess::Validate => "Validate patch",
            PatchProcess::Delete => "Delete patch",
            PatchProcess::Rename => "Rename patch",
        },
        Process::Mod(process) => match process {
            ModProcess::Add => "Add mod to vault",
            ModProcess::Remove => "Remove mod from vault",
            ModProcess::Validate => "Validate mod",
            ModProcess::Export => "Export mod",
            ModProcess::Import => "Import mod",
            ModProcess::Clean => "Cleaning vault",
        },
        Process::Comp(process) => match process {
            CompProcess::Serialize => "Serialize",
            CompProcess::Deserialize => "Deserialize",
            CompProcess::Compress => "Compress",
            CompProcess::Decompress => "Decompress",
        },
    }
    .to_owned()
}

fn match_message(message: &Message) -> String {
    match message {
        Message::Transparent(content) => content.to_owned(),
        Message::Hash(message) => match message {
            HashMessage::HashFilesStatus { path } => {
                format!("Mod file path: '{}'", path.display().to_string().cyan())
            }
        },
        Message::Instance(message) => match message {
            InstanceMessage::LinkSuccess { instance } => format!(
                "Linked '{}' with bundle '{}' and patch '{}'",
                instance.name.cyan(),
                instance.bundle.cyan(),
                instance.patch.cyan()
            ),
            InstanceMessage::DetectSuccess { bundle, patch } => {
                format!(
                    "Detected bundle '{}' and patch '{}'",
                    bundle.cyan(),
                    patch.cyan()
                )
            }
            InstanceMessage::ApplySuccess { bundle, patch } => format!(
                "Applied patch '{}' from bundle '{}'",
                patch.cyan(),
                bundle.cyan()
            ),
            InstanceMessage::ValidateSuccess { name } => {
                format!("Validated instance '{}'", name.cyan())
            }
            InstanceMessage::ValidateStatus { name } => {
                format!("{}", name.cyan())
            }
            InstanceMessage::Select => "Please select an instance".to_owned(),
            InstanceMessage::Option { instance } => {
                format!(
                    "{} [{}, {}]",
                    instance.name.cyan(),
                    instance.bundle.cyan(),
                    instance.path.display().to_string().cyan()
                )
            }
        },
        Message::Bundle(message) => match message {
            BundleMessage::AddModFileStatus { path, hash } => {
                format!(
                    "Mod file path: '{}' ['{}']",
                    path.display().to_string().cyan(),
                    hash.yellow()
                )
            }
            BundleMessage::CreateSuccess { bundle } => {
                format!("Created bundle '{}'", bundle.name.cyan())
            }
            BundleMessage::ValidateSuccess { name } => {
                format!("Validated bundle '{}'", name.cyan())
            }
            BundleMessage::ValidateStatus { name } => format!("{}", name.cyan()),
            BundleMessage::ExportSuccess { bundle, path } => {
                format!("Exported bundle '{}' to file '{}'", bundle, path.display())
            }
            BundleMessage::Select => "Please select a bundle".to_owned(),
            BundleMessage::Option { bundle } => {
                format!(
                    "{} ({})",
                    bundle.name.cyan(),
                    format_string_option(&bundle.description)
                )
            }
        },
        Message::Patch(message) => match message {
            PatchMessage::SimulateStatus { name } => format!("Patch: '{}'", name.cyan()),
            PatchMessage::CreateSuccess { patch } => format!(
                "Created patch '{}' for bundle '{}' depending on patch '{}'",
                patch.name.cyan(),
                patch.bundle.cyan(),
                patch.dependency.cyan()
            ),
            PatchMessage::ExcludeSuccess { hash } => {
                format!("Excluded mod with hash '{}'", hash.yellow())
            }
            PatchMessage::GenerateSuccess { name, instance } => format!(
                "Generated patch '{}' from changes to '{}'",
                name.cyan(),
                instance.cyan()
            ),
            PatchMessage::HashModFileStatus { path, hash } => format!(
                "Mod file path: '{}' ['{}']",
                path.display().to_string().cyan(),
                hash.yellow()
            ),
            PatchMessage::IncludeSuccess { hash } => {
                format!("Included mod with hash '{}'", hash.yellow())
            }
            PatchMessage::ValidateSuccess { name } => format!("Validated patch '{}'", name.cyan()),
            PatchMessage::ValidateStatus { bundle, name } => {
                format!("{} for {}", name.cyan(), bundle.cyan())
            }
            PatchMessage::DeleteSuccess { name, bundle } => format!(
                "Deleted patch '{}' from bundle '{}'",
                name.cyan(),
                bundle.cyan()
            ),
            PatchMessage::RenameSuccess {
                name,
                bundle,
                new_name,
            } => format!(
                "Renamed patch '{}' for bundle '{}' to '{}'",
                name.cyan(),
                bundle.cyan(),
                new_name.cyan()
            ),
        },
        Message::Mod(message) => match message {
            ModMessage::AddSuccess { value } => {
                format!("Added mod with hash '{}' to vault", value.hash.yellow())
            }
            ModMessage::RemoveStatus { hash } => {
                format!("Removed mod with hash '{}' from vault", hash.yellow())
            }
            ModMessage::ValidateSuccess { hash } => {
                format!("Validated mod with hash '{}'", hash.yellow())
            }
            ModMessage::RemoveConfirm { value } => format!(
                "Do you want to remove this mod from the vault?\n{} ({}, {}, {}) [{}]",
                value.meta.name.bold().cyan(),
                value.meta.id.bold(),
                value.meta.loader,
                format_string_option(&value.meta.minecraft_version),
                value.hash.yellow()
            ),
            ModMessage::RemoveSelect => String::from(
                "Multiple mods match the supplied filters. Please select the one you wish to remove:",
            ),
            ModMessage::RemoveOption { value } => format!(
                "{} ({}, {}, {}) [{}]",
                value.meta.name.bold().cyan(),
                value.meta.id.bold(),
                format_string_option(&value.meta.minecraft_version),
                value.meta.loader,
                value.hash.yellow()
            ),
            ModMessage::ValidateStatus { hash } => format!("{}", hash.yellow()),
            ModMessage::ExportSuccess { hash, path } => format!(
                "Exported mod with hash '{}' to file '{}'",
                hash,
                path.display()
            ),
            ModMessage::ImportSuccess { hash, path } => format!(
                "Imported mod with hash '{}' from file '{}'",
                hash,
                path.display()
            ),
            ModMessage::CleanSuccess { values } => {
                let mut msg = format!(
                    "Cleaned vault, removed {} mods:",
                    values.len().to_string().purple()
                );

                for (hash, id) in values {
                    msg = msg + &format!("\n\t'{}' [{}]", id.cyan(), hash.yellow())
                }

                msg
            }
            ModMessage::CleanStatus { hash, id } => format!("'{}' [{}]", id.cyan(), hash.yellow()),
        },
        Message::Comp(_message) => "".to_owned(),
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
                    multi_progress.remove(&progress_bar);
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
            Event::ProgressTick {
                process,
                message,
                increment,
            } => {
                let progress_bar = processes
                    .get(&process)
                    .ok_or(Error::Event(EventError::ProcessNotFound))?;

                progress_bar.set_message(match_message(&message));
                progress_bar.inc(increment);
            }
            Event::ProgressFinish { process, message } => {
                let progress_bar = processes
                    .get(&process)
                    .ok_or(Error::Event(EventError::ProcessNotFound))?;
                progress_bar.set_style(ProgressStyle::with_template(&format!(
                    "{} [{}]: {{msg}}",
                    "✓".bold().green(),
                    match_process(&process).bold().green()
                ))?);

                match message {
                    None => {
                        progress_bar.finish_and_clear();
                        multi_progress.remove(progress_bar);
                    }
                    Some(message) => progress_bar.finish_with_message(match_message(&message)),
                }
            }
            Event::Confirm { tx, message } => {
                let result = multi_progress.suspend(|| {
                    Confirm::new(&match_message(&message))
                        .with_default(false)
                        .prompt()
                })?;

                tx.send(result)?;
            }
            Event::Select {
                tx,
                message,
                options,
            } => {
                let index = multi_progress.suspend(|| {
                    let option_strings = options.iter().map(match_message).collect::<Vec<String>>();
                    let result =
                        Select::new(&match_message(&message), option_strings.clone()).prompt()?;

                    option_strings
                        .iter()
                        .position(|msg| &result == msg)
                        .ok_or(Error::Event(EventError::InvalidSelection))
                })?;

                tx.send(index)?;
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
        let mut conn = db::init()?;
        let transaction = conn.transaction()?;
        let result = match_command(&cli.command, &transaction, &tx);
        if result.is_ok() {
            transaction.commit()?
        };

        result
    });

    if let Err(error) = fallible(rx, thread) {
        println!("{}", StatusOutput::new(Status::Error, error.to_string()));
    }
}
