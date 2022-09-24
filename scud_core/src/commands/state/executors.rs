use std::process::Command;

use colored::Colorize;

use crate::{
    diagnostics::{log_diagnostic, DiagnosticKind},
    state::helpers::{
        display_all_staged_changes, display_all_unstaged_changes,
        display_all_untracked_changes, display_commit_state, display_current_branch,
        display_state_header, get_commit_diff,
    },
};

pub fn execute_state_info() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command: "state",
        description: "This command is intended to display repository metadata \
                      including branching information as well as file changes in a \
                      concise, human-readable format.",
    });
    log_diagnostic(DiagnosticKind::VCSInfo {
        command_name: "state",
        git_command: &format!(
            "{} {}",
            "git status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        mercurial_command: &format!(
            "{} {}",
            "hg status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        breezy_command: &format!(
            "{} {}",
            "bzr status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
    });
}

pub fn execute_state_git() {
    display_state_header();

    display_current_branch();

    display_all_staged_changes();

    display_all_unstaged_changes();

    display_all_untracked_changes();

    // get status diff (e.g. git status -sb)
    // parse number of commits ahead and behind
    // if behind, print warning

    match Command::new("git").args(["status", "-sb"]).output() {
        Ok(output) => {
            let status_diff = String::from_utf8_lossy(&output.stdout).to_string();
            let ahead_behind = get_commit_diff(status_diff);
            display_commit_state(ahead_behind);
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting status diff (git)",
            body: &format!("{}", error),
        }),
    }
}

pub fn execute_state_mercurial() {}

pub fn execute_state_breezy() {}
