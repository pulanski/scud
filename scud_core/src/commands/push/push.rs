use std::time::SystemTime;
use std::process::Command;
use crate::{
    cli::Push,
    logging::general::{
      log_dry_run_note,
      log_execution_time,
    },
};

pub fn push_command(push_options: Push, start_time: SystemTime) {

    // check for global config file

    // check for vcs option passed in push_options
    // will override global config file

    if push_options.dry_run {
        execute_push_dry_run();
        log_dry_run_note();
    } else {
        execute_push();
    }

    match push_options.dry_run {
        true => {

        }
        false => {

        }
    }

    log_execution_time(start_time);
}

fn execute_push() {
    match Command::new("git")
        .arg("push")
        .status() {
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
    match Command::new("git")
        .arg("push")
        .arg("--dry-run")
        .status() {
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

// TODO add helpful diagnostics for when
// push occurs but no files are committed
// highlight green for staged files
// Hint: Consider committing the following files
// highlight yellow for unstaged files
// consider staging and committing the following files
