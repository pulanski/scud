use colored::Colorize;

use crate::{
    cli::{
        Commit,
        VCS,
    },
    commands::commit::{
        executors::{
            execute_commit_breezy,
            execute_commit_git,
            execute_commit_mercurial,
        },
        helpers::process_commit_message,
    },
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    helpers::detect_vcs,
    logging::{
        general::log_dry_run_note,
        helpers::{
            black_comma,
            black_period,
            bright_yellow_backtick,
        },
    },
};

/// Defining an enum to represent the supported Commit Message Specifications:
///     * Angular
///     * Conventional
///     * None
///     * Unknown
pub enum CommitMessageFormat {
    Angular,
    Conventional,
    None,
    Unknown,
}

/// It executes the commit command with the given options
///
/// Arguments:
///
/// * `commit_options`: This is the struct that contains the options that   were
///   passed to the command via user input.
pub fn commit_command(commit_options: Commit) {
    if commit_options.dry_run {
        execute_commit_dry_run();
        log_dry_run_note();
    } else if commit_options.info {
        execute_commit_info();
    } else {
        execute_commit();
    }
}

/// It detects the version control system, and then calls the appropriate
/// function to execute the commit
fn execute_commit() {
    let vcs = detect_vcs();

    match vcs {
        // Each executor checks to make sure there are staged files
        // before executing the commit command.
        // If there are no staged files, it logs a warning and exits.
        VCS::Git => execute_commit_git(),
        VCS::Mercurial => execute_commit_mercurial(),
        VCS::Breezy => execute_commit_breezy(),
    }
}

fn execute_commit_dry_run() {
    let _commit_message = process_commit_message();
    log_diagnostic(DiagnosticKind::DryRun { command: "commit" });

    // println!(
    //     "\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
    //     " NOTE ".on_bright_yellow(),
    //     "  No commit was executed".yellow().italic(),
    //     ".".black(),
    //     " To ".yellow().italic(),
    //     "properly commit".bright_yellow().italic(),
    //     " to the current branch".yellow().italic(),
    //     ",".black(),
    //     " rerun".bright_yellow().italic(),
    //     " the ".yellow().italic(),
    //     "`".bright_yellow(),
    //     "scud commit".green(),
    //     "`".bright_yellow(),
    //     " command ".yellow().italic(),
    //     "without".bright_yellow().italic(),
    //     " the ".yellow().italic(),
    //     "\"".black().italic(),
    //     "--".bright_yellow().italic(),
    //     "dry-run".yellow().italic(),
    //     "\"".black().italic(),
    //     " flag".yellow().italic(),
    //     ".".black(),
    // );
}

fn execute_commit_info() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "Commit under the hood",
    });
}
