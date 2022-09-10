use std::process::Command;

use crate::{
    commands::commit::helpers::{
        check_for_staged_files,
        process_commit_message,
    },
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
};

use colored::Colorize;

/// Takes the commit message from the user
/// and then logs a diagnostic message
/// indicating the usage of the dry-run option.
pub fn execute_commit_dry_run() {
    process_commit_message();
    log_diagnostic(DiagnosticKind::DryRun { command: "commit" });
}

pub fn execute_commit_info() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command:     "commit",
        description: "This command is used to construct a human-readable commit \
                      message and commit staged changes to the current branch in \
                      the local repository.",
    });
    log_diagnostic(DiagnosticKind::VCSInfo {
        command_name:      "commit",
        git_command:       &format!(
            "{} {}",
            "git commit -m \"<generated message>\"",
            "(generated commit message comes from user flow w/ CLI)".bright_yellow()
        ),
        mercurial_command: &format!(
            "{} {}",
            "hg commit -m \"<generated message>\"",
            "(generated commit message comes from user flow w/ CLI)".bright_yellow()
        ),
        breezy_command:    &format!(
            "{} {}",
            "bzr commit -m \"<generated message>\"",
            "(generated commit message comes from user flow w/ CLI)".bright_yellow()
        ),
    });
}

//////////////////////////////////////////////////////////
// Functions handle the execution of each the supported //
// underlying VCS implementations of the commit command //
//////////////////////////////////////////////////////////

/// Checks for staged files,
/// processes the commit message,
/// and then executes the `git commit` command
/// with the generated message.
pub fn execute_commit_git() {
    check_for_staged_files();

    let commit_message = process_commit_message();

    match Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let _successful_commit_output =
                stdout.split("\n").collect::<Vec<&str>>();
            // If the commit was successful, print the output to the user
            println!("\n");
            // println!("{}",
            // String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "git commit failed",
            body:    &format!("{}", error),
        }),
    }
}

pub fn execute_commit_mercurial() {
    let _commit_message = process_commit_message();

    todo!("execute_commit_mercurial");
}

pub fn execute_commit_breezy() {
    let _commit_message = process_commit_message();

    todo!("execute_commit_breezy");
}
