use crate::logging::general::log_execution_time;
use crate::{
    cli::cli::Push,
    commands::push::executors::execute_push_dry_run,
};
use std::process::Command;
use std::time::SystemTime;

pub fn push_command(push_options: Push, start_time: SystemTime) {
    if push_options.dry_run {
        execute_push_dry_run();
    } else if push_options.info {
        execute_push_info();
    } else {
        execute_push();
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

pub fn execute_push_info() {}

// TODO add helpful diagnostics for when
// push occurs but no files are committed
// highlight green for staged files
// Hint: Consider committing the following files
// highlight yellow for unstaged files
// consider staging and committing the following files
