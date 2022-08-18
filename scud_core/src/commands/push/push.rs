use crate::diagnostics::{
    log_diagnostic,
    DiagnosticKind,
};
use crate::{
    cli::Push,
    logging::general::log_execution_time,
};
use std::process::Command;
use std::time::SystemTime;

pub fn push_command(push_options: Push, start_time: SystemTime) {
    // check for global config file

    // check for vcs option passed in push_options
    // will override global config file

    if push_options.dry_run {
        execute_push_dry_run();
    } else {
        execute_push();
    }

    match push_options.dry_run {
        true => {}
        false => {}
    }

    log_execution_time(start_time);
}

fn execute_push() {
    match Command::new("git").arg("push").status() {
        Ok(status) => {
            if !status.success() {
                panic!("Failed to push files");
            }
        }
        Err(error) => {
            panic!("Failed to push files: {}", error);
        }
    }
}

fn execute_push_dry_run() {
    match Command::new("git").arg("push").arg("--dry-run").status() {
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

// TODO add helpful diagnostics for when
// push occurs but no files are committed
// highlight green for staged files
// Hint: Consider committing the following files
// highlight yellow for unstaged files
// consider staging and committing the following files
