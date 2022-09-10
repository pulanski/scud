use std::time::SystemTime;

use colored::Colorize;

use crate::{
    branching::{
        Branch,
        BranchCommands,
    },
    commands::branch::list::list::branch_list_command,
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    general::log_execution_time,
};

pub fn process_branch_commands(branch_commands: Branch, start_time: SystemTime) {
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
            branch_command(branch_commands.info, start_time);
            // get_branch_command(start_time);
        }
    }
}

pub fn branch_command(info: bool, start_time: SystemTime) {
    if info {
        execute_branch_info();
    } else {
        execute_branch();
    }
    log_execution_time(start_time);
}

pub fn execute_branch_info() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command:     "branch",
        description: "This command is used to list branches in both the local and \
                      remote repositories.",
    });
    log_diagnostic(DiagnosticKind::VCSInfo {
        command_name:      "branch",
        git_command:       &format!(
            "{} {}",
            "git branch -A",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        mercurial_command: &format!(
            "{} {}",
            "hg branches",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        breezy_command:    &format!(
            "{} {}",
            "bzr branches",
            "(along with other commands for more rich output)".bright_yellow()
        ),
    });
}

pub fn execute_branch() {}

// pub fn get_branch_command(start_time: SystemTime) {
//     let branch_subcommand_options = &[
//         "List [alias: ls]: Lists all branches in the current repository (both
// \          local and remote).",
//         "Rename [alias: rn]: Renames a branch",
//         "Delete [alias: del]: Deletes a branch in the current repository
// (local or \          remote).",
//     ];
//
//     let selected_subcommand =
// FuzzySelect::with_theme(&ColorfulTheme::default())
//         .with_prompt(format!(
//             "{}{}{}",
//             "Select the ".bright_yellow().italic(),
//             "branch".yellow().italic(),
//             " subcommand that you wish to use: "
//                 .bright_yellow()
//                 .italic()
//         ))
//         .default(0)
//         .items(&branch_subcommand_options[..])
//         .interact()
//         .unwrap();
//
//     clearscreen::clear().expect("failed to clear screen");
//
//     match selected_subcommand {
//         0 => {
//             branch_list_command(BranchList { info: false }, start_time);
//         }
//         1 => {
//             log_diagnostic(DiagnosticKind::WorkInProgress {
//                 feature: "branching (rename)",
//             });
//         }
//         2 => {
//             log_diagnostic(DiagnosticKind::WorkInProgress {
//                 feature: "branching (delete)",
//             });
//         }
//         _ => unreachable!(),
//     }
// }
