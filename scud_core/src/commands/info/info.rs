use std::time::SystemTime;

use colored::Colorize;
use dialoguer::{
    theme::ColorfulTheme,
    FuzzySelect,
};

use crate::{
    commands::info::{
        codebase::codebase::info_codebase_command,
        system::system::info_system_command,
    },
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    information::{
        Codebase,
        Info,
        InfoCommands,
    },
};

pub fn process_info_commands(info_commands: Info, start_time: SystemTime) {
    {
        match info_commands.command {
            Some(info_command) => match info_command {
                InfoCommands::Codebase(codebase_options) => {
                    info_codebase_command(codebase_options, start_time);
                }
                InfoCommands::System(system_options) => {
                    info_system_command(system_options, start_time);
                }
            },
            None => {
                get_branch_command(start_time);
            }
        }
    }
}

pub fn get_branch_command(start_time: SystemTime) {
    let branch_subcommand_options = &[
        "Codebase [alias: cb]: Information about the contents of the codebase \
         within the
    current working directory",
        "System [alias: sys]: Information about the system on which scud is \
         running [alias: sys] ",
    ];

    let selected_subcommand = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}{}{}",
            "Select the ".bright_yellow().italic(),
            "info".yellow().italic(),
            " subcommand that you wish to use: "
                .bright_yellow()
                .italic()
        ))
        .default(0)
        .items(&branch_subcommand_options[..])
        .interact()
        .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    match selected_subcommand {
        0 => {
            info_codebase_command(Codebase { info: false }, start_time);
        }
        1 => {
            log_diagnostic(DiagnosticKind::WorkInProgress {
                feature: "system information",
            });
        }
        _ => unreachable!(),
    }
}
