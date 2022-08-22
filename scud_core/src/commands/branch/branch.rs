use std::time::SystemTime;

use colored::Colorize;
use dialoguer::{
    theme::ColorfulTheme,
    FuzzySelect,
};

use crate::{
    branching::{
        Branch,
        BranchCommands,
        BranchList,
    },
    commands::branch::list::list::branch_list_command,
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
};

pub fn process_branch_commands(branch_commands: Branch, start_time: SystemTime) {
    {
        match branch_commands.command {
            Some(branch_command) => match branch_command {
                BranchCommands::List(branch_list_options) => {
                    branch_list_command(branch_list_options, start_time);
                }
                _ => {
                    log_diagnostic(DiagnosticKind::WorkInProgress {
                        feature: "branching",
                    });
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
        "List [alias: ls]: Lists all branches in the current repository (both \
         local and remote).",
        "Rename [alias: rn]: Renames a branch",
        "Delete [alias: del]: Deletes a branch in the current repository (local or \
         remote).",
    ];

    let selected_subcommand = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}{}{}",
            "Select the ".bright_yellow().italic(),
            "branch".yellow().italic(),
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
            branch_list_command(BranchList { info: false }, start_time);
        }
        1 => {
            log_diagnostic(DiagnosticKind::WorkInProgress {
                feature: "branching (rename)",
            });
        }
        2 => {
            log_diagnostic(DiagnosticKind::WorkInProgress {
                feature: "branching (delete)",
            });
        }
        _ => unreachable!(),
    }
}
