use crate::{cli::cli::Push, commands::push::executors::execute_push_dry_run};
use std::process::Command;

pub fn push_command(push_options: Push) {
    if push_options.dry_run {
        execute_push_dry_run();
    } else if push_options.info {
        execute_push_info();
    } else {
        execute_push();
    }
}

fn execute_push() {
    // TODO: pipe output
    // EXAMPLE output to parse:
    //     Enumerating objects: 194, done.
    // Counting objects: 100% (192/192), done.
    // Delta compression using up to 10 threads
    // Compressing objects: 100% (136/136), done.
    // Writing objects: 100% (152/152), 24.93 KiB | 6.23 MiB/s, done.
    // Total 152 (delta 95), reused 0 (delta 0), pack-reused 0
    // remote: Resolving deltas: 100% (95/95), completed with 27 local objects.
    // To https://github.com/pulanski/scud.git
    //    2b03205..5f2d9da  main -> main

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
