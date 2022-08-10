use crate::{
    cli::{Commit, VCS},
    commands::commit::{
        executors::{execute_commit_breezy, execute_commit_git, execute_commit_mercurial},
        helpers::process_commit_message,
    },
    diagnostics::{log_diagnostic, DiagnosticKind},
    helpers::detect_vcs,
    logging::general::log_dry_run_note,
};

pub enum CommitMessageFormat {
    Angular,
    Conventional,
    None,
    Unknown,
}

pub fn commit_command(commit_options: Commit) {
    if commit_options.dry_run {
        // TODO maybe refactor here
        execute_commit_dry_run();
        log_dry_run_note();
    } else if commit_options.info {
        execute_commit_info();
    } else {
        execute_commit();
    }
}

fn execute_commit() {
    // TODO fix commit execution pipeline

    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_commit_git(),
        VCS::Mercurial => execute_commit_mercurial(),
        VCS::Breezy => execute_commit_breezy(),
    }
}

fn execute_commit_dry_run() {
    let _commit_message = process_commit_message();
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "Commit dry run",
    });
}

fn execute_commit_info() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "Commit under the hood",
    });
}
