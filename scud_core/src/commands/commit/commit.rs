use crate::{
    cli::cli::{
        Commit,
        VCS,
    },
    commands::commit::executors::{
        execute_commit_breezy,
        execute_commit_dry_run,
        execute_commit_git,
        execute_commit_info,
        execute_commit_mercurial,
    },
    helpers::detect_vcs,
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

/// Executes the commit command with the given options
///
/// Arguments:
///
/// * `commit_options`: Struct that contains the options that were passed to the
///   command via user input (e.g. dry-run, info).
pub fn commit_command(commit_options: Commit) {
    if commit_options.dry_run {
        execute_commit_dry_run();
    } else if commit_options.info {
        execute_commit_info();
    } else {
        execute_commit();
    }
}

/// Detects the version control system, and then calls the appropriate
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
