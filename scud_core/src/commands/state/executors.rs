use std::process::Command;

use colored::Colorize;

use crate::diagnostics::{
    log_diagnostic,
    DiagnosticKind,
};

pub fn execute_state_info() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command:     "state",
        description: "This command is intended to display repository metadata \
                      including branching information as well as file changes in a \
                      concise, human-readable format.",
    });
    log_diagnostic(DiagnosticKind::VCSInfo {
        command_name:      "state",
        git_command:       &format!(
            "{} {}",
            "git status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        mercurial_command: &format!(
            "{} {}",
            "hg status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
        breezy_command:    &format!(
            "{} {}",
            "bzr status",
            "(along with other commands for more rich output)".bright_yellow()
        ),
    });
}

pub fn execute_state_git() {
    println!(
        "\n{} {}",
        " Git ".black().italic().on_bright_yellow(),
        "repository status".yellow().italic(),
    );

    println!("\n{}\n", " CHANGES TO FILES: ".black().on_cyan());

    // get_all_staged_changes();

    // Get changes staged for commit
    // git diff --name-only --cached
    match Command::new("git")
        .args(["diff", "--name-only", "--cached"])
        .output()
    {
        Ok(output) => {
            let staged_changes = String::from_utf8_lossy(&output.stdout).to_string();

            if staged_changes.len() > 0 {
                println!("  {}\n", "Changes staged for commit: ".black().italic());
                // TODO IDEA ?? make outputted files clickable
                // where click links to file in default text editor

                for staged_change in staged_changes.lines() {
                    println!(
                        "    {}  {}",
                        " M ".black().on_green(),
                        staged_change.to_string().bright_green().italic()
                    );
                }
                println!();
            } else {
                println!(
                    "  {}\n",
                    "No changes staged for commit...".black().italic()
                );
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting staged changes (git)",
            body:    &format!("{}", error),
        }),
    }

    // get only unstaged files
    // git ls-files --modified --exclude-
    // match Command::new("git")
    match Command::new("git")
        .args(["ls-files", "--modified", "--exclude-standard"])
        .output()
    {
        Ok(output) => {
            let unstaged_files = String::from_utf8_lossy(&output.stdout).to_string();

            if unstaged_files.len() > 0 {
                println!("  {}\n", "Unstaged files: ".black().italic());

                for unstaged_file in unstaged_files.lines() {
                    println!(
                        "    {}  {}",
                        " M ".black().on_red(),
                        unstaged_file.to_string().bright_red().italic()
                    );
                }
                println!();
            } else {
                println!("  {}\n", "No unstaged files...".black().italic());
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting unstaged files (git)",
            body:    &format!("{}", error),
        }),
    }

    // get only untracked files
    // git ls-files --others --exclude-standard
    match Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .output()
    {
        Ok(output) => {
            let untracked_files =
                String::from_utf8_lossy(&output.stdout).to_string();

            if untracked_files.len() > 0 {
                println!("  {}\n", "Untracked files: ".black().italic());

                for untracked_file in untracked_files.lines() {
                    println!(
                        "    {}  {}",
                        " ?? ".black().on_red(),
                        untracked_file.to_string().bright_red().italic()
                    );
                }
                println!();
            } else {
                println!("  {}\n", "No untracked files...".black().italic());
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting untracked files (git)",
            body:    &format!("{}", error),
        }),
    }

    // get status diff (e.g. git status -sb)
    // parse number of commits ahead and behind
    // if behind, print warning

    match Command::new("git").args(["status", "-sb"]).output() {
        Ok(output) => {
            let status_diff = String::from_utf8_lossy(&output.stdout).to_string();
            let mut status = status_diff.split("\n");

            let branch_metadata = status.next().unwrap();
            println!("{}", branch_metadata.bright_black().italic());
            let branch_metadata_tokens = branch_metadata.split('[');

            let ahead_behind_raw = branch_metadata_tokens.last().unwrap();
            let ahead_behind = ahead_behind_raw.split(']').next().unwrap();
            println!("{}", ahead_behind.bright_black().italic());

            let mut ahead_behind_split = ahead_behind.split(',');

            // [behind 1]
            // [ahead 1]
            // [ahead 1, behind 1]
            let ahead = ahead_behind_split.next().unwrap();
            let behind = ahead_behind_split.next().unwrap();
            let ahead_int = ahead.parse::<i32>().unwrap();
            let behind_int = behind.parse::<i32>().unwrap();
            if ahead_int > 0 {
                println!(
                    "  {}{}\n",
                    "You are commits ahead of the
              remote..."
                        .black()
                        .italic(),
                    ahead_int
                );
            }
            if behind_int > 0 {
                println!(
                    "  {}{}\n",
                    "You are commits behind the
     remote..."
                        .black()
                        .italic(),
                    behind_int
                );
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting status diff (git)",
            body:    &format!("{}", error),
        }),
    }
}

pub fn execute_state_mercurial() {}

pub fn execute_state_breezy() {}
