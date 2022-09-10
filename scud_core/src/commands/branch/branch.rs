use std::time::SystemTime;

use colored::Colorize;

use crate::{
    branching::{
        Branch,
        BranchCommands,
    },
    cli::cli::VCS,
    commands::branch::executors::{
        execute_branch_breezy,
        execute_branch_git,
        execute_branch_mercurial,
    },
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    helpers::detect_vcs,
    logging::{
        general::log_execution_time,
        helpers::bright_yellow_backtick,
    },
};

pub fn process_branch_commands(branch_commands: Branch, start_time: SystemTime) {
    match branch_commands.command {
        Some(branch_command) => match branch_command {
            //       BranchCommands::List(branch_list_options) => {
            //           branch_list_command(branch_list_options, start_time);
            //       }
            BranchCommands::Rename(branch_rename_options) => {
                //TODO branch_rename_command(branch_rename_options,
                // start_time);
            }
            _ => {
                log_diagnostic(DiagnosticKind::WorkInProgress {
                    feature: "branching",
                });
            }
        },
        None => {
            branch_command(branch_commands.info, start_time);
        }
    }
}

/// Lists all branches both local and remote in the current repository.
pub fn branch_command(info: bool, start_time: SystemTime) {
    if info {
        execute_branch_info();
    } else {
        execute_branch();
    }
    log_execution_time(start_time);
}

/// Lists the commands that scud runs under the hood to list all branches
/// in the current repository (both local and remote).
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

pub fn execute_branch() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_branch_git(),
        VCS::Mercurial => execute_branch_mercurial(),
        VCS::Breezy => execute_branch_breezy(),
    }

    log_diagnostic(DiagnosticKind::Tip {
        body: &format!(
            "{} {}{}{} {} {}{}{} {}",
            "Use".yellow(),
            bright_yellow_backtick(),
            "scud branch create".green().italic(),
            bright_yellow_backtick(),
            "to create a new local branch or use".yellow(),
            bright_yellow_backtick(),
            "scud move".green().italic(),
            bright_yellow_backtick(),
            "to move between branches in the current local repository".yellow(),
        ),
    });
}
