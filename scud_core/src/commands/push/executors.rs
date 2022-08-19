use std::process::Command;

use crate::diagnostics::{
    log_diagnostic,
    DiagnosticKind,
};

pub fn execute_push_dry_run() {
    match Command::new("git").args(["push", "--dry-run"]).status() {
        Ok(status) => {
            if !status.success() {
                panic!("Failed to push files");
            }
        }
        Err(error) => {
            panic!("Failed to push files: {}", error);
        }
    }

    log_diagnostic(DiagnosticKind::DryRun { command: "push" });
}
