use std::process::Command;

use crate::{
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    state::executors::execute_state_git,
};

pub fn execute_stage_dry_run() {
    log_diagnostic(DiagnosticKind::DryRun { command: "stage" });
}

pub fn execute_stage_info() {
    log_diagnostic(DiagnosticKind::CommandInfo {
        command:     "stage",
        description: "This command is intended to stage all modified files in the \
                      current local repository ensuring they are ready to be \
                      committed. It is smart enough to detect the underlying \
                      version control system in which it is being run and execute \
                      the associated commands. Additionally, it can be run within \
                      any deeply nested subdirectory of the local repository root \
                      and will stage all files.",
    });
    log_diagnostic(DiagnosticKind::VCSInfo {
        command_name:      "stage",
        git_command:       "git add -A",
        mercurial_command: "Algorithm pseudocode:\n\t1. (Navigate to root of \
                            repository)\n\t2. hg add .\n\t3. (Navigate back to \
                            invocation directory)",
        breezy_command:    "Algorithm pseudocode:\n\t1. (Navigate to root of \
                            repository)\n\t2. bzr add .\n\t3. (Navigate back to \
                            invocation directory)",
    });
}

// TODO add doc comments

pub fn execute_stage_git() {
    // check for unstaged untracked files

    

    match Command::new("git").args(["add", "-A"]).output() {
        Ok(_) /* git add -A doesn't emit any output */ => {
            execute_state_git();
        }
        Err(error) => log_diagnostic(
            DiagnosticKind::Error {
                subject: "failed to stage (git)",
                body: &format!("{}", error)
            }
        )
    }
}

pub fn execute_stage_mercurial() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud stage (mercurial)",
    });
}

pub fn execute_stage_breezy() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud stage (breezy)",
    });
}
