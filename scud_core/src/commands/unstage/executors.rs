use std::process::Command;

use crate::{
    diagnostics::{log_diagnostic, DiagnosticKind},
    state::executors::execute_state_git,
};

pub fn execute_unstage_git() {
    match Command::new("git").arg("reset").output() {
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

pub fn execute_unstage_mercurial() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud unstage (mercurial)",
    });
}

pub fn execute_unstage_breezy() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud unstage (breezy)",
    });
}
