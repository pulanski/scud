use std::process::Command;

use crate::{
    diagnostics::{log_diagnostic, DiagnosticKind},
    state::executors::execute_state_git,
};

pub fn execute_stage_git() {
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
