use std::process::Command;

use colored::Colorize;

use crate::diagnostics::{log_diagnostic, DiagnosticKind};

pub fn execute_branch_git() {
    println!(
        "\n{} {}",
        " Git ".black().italic().on_bright_yellow(),
        "repository status".yellow().italic(),
    );

    match Command::new("git").args(&["branch", "-a"]).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();

            // Log to user BRANCH INFORMATION:
            println!("\n{}\n", " BRANCH INFORMATION: ".black().on_cyan());

            // Collect local branches
            let local_branches = stdout
                .split("\n")
                .filter(|line| !line.contains("remotes") && !line.is_empty())
                .collect::<Vec<&str>>();

            // Log local branches
            println!("  {}\n", " LOCAL ".black().on_magenta());
            let mut i = 0;
            let local_branches_len = local_branches.len();
            for mut branch in local_branches {
                branch = branch.trim();
                i += 1;

                // check if on first or last branch
                if i == 1 {
                    print!("  {}  ", "/-".magenta());
                } else if i == local_branches_len {
                    print!("  {}  ", "\\-".magenta());
                } else {
                    print!("  {}  ", "--".magenta());
                }

                // check if branch is current branch
                if branch.contains('*') {
                    print!("{}", "**".red());
                    let branch_name = branch.split('*').last().unwrap();
                    branch = branch_name.trim();
                    print!(" {}", branch.cyan());
                    println!("{}{}", " <-".green(), " ACTIVE ".red());
                } else {
                    println!("{}", branch.cyan());
                }

                // check if branch is ahead or behind
                // TODO move this to state
                //if branch.contains('[') {
                //    println!(
                //        "{}{}",
                //        " <-".black().italic(),
                //        " AHEAD ".bright_yellow()
                //    );
                //} else if branch.contains(']') {
                //    println!(
                //        "{}{}",
                //        " <-".black().italic(),
                //        " BEHIND ".bright_yellow()
                //    );
                //}

                // Get the latest commit hash for the branch
                let mut latest_commit_hash = String::new();
                match Command::new("git")
                    .args(&["log", "-1", "--pretty=%H", branch])
                    .output()
                {
                    Ok(output) => {
                        latest_commit_hash =
                            String::from_utf8_lossy(&output.stdout).to_string();
                    }
                    Err(error) => {
                        log_diagnostic(DiagnosticKind::Error {
                            subject: "git log",
                            body: &format!("{}", error),
                        });
                    }
                }

                // print the separator
                if i != local_branches_len {
                    print!("  {}", "|".blue());
                } else {
                    print!("  {}", " ".blue());
                }

                // print latest commit
                if latest_commit_hash.is_empty() {
                    print!("  {}", "No commits".bright_black().italic());
                } else {
                    print!(
                        "  {} {}",
                        "Latest commit:".bright_yellow().italic(),
                        latest_commit_hash.white().italic()
                    );
                }
            }

            // Collect remote branches
            let remote_branches = stdout
                .split("\n")
                .filter(|line| line.contains("remotes"))
                .collect::<Vec<&str>>();

            // Log remote branches
            println!("\n  {}\n", " REMOTE ".black().on_magenta());
            let mut i = 0;
            let remote_branches_len = remote_branches.len();
            for mut branch in remote_branches {
                branch = branch.trim();
                let branch_raw = branch.replace("remotes/origin/", "");
                branch = branch_raw.as_str();
                i += 1;

                // check if on first or last branch
                if i == 1 {
                    print!("  {}  ", "/-".magenta());
                } else if i == remote_branches_len {
                    print!("  {}  ", "\\-".magenta());
                } else {
                    print!("  {}  ", "--".magenta());
                }

                println!("{}", branch.cyan());
                // print the separator
                if i != remote_branches_len {
                    print!("  {}", "|".blue());
                }

                // Get the latest commit hash for the branch
                let mut latest_commit_hash = String::new();
                match Command::new("git")
                    .args(&["log", "-1", "--pretty=%H", branch])
                    .output()
                {
                    Ok(output) => {
                        latest_commit_hash =
                            String::from_utf8_lossy(&output.stdout).to_string();
                    }
                    Err(error) => {
                        log_diagnostic(DiagnosticKind::Error {
                            subject: "git log",
                            body: &format!("{}", error),
                        });
                    }
                }

                // print latest commit
                if latest_commit_hash.is_empty() {
                    println!("  {}", "No commits".bright_black().italic());
                } else {
                    println!(
                        "     {} {}",
                        "Latest commit:".bright_yellow().italic(),
                        latest_commit_hash.white().italic()
                    );
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    // match Command::new("git").arg("reset").output() {
    //     Ok(_) /* git add -A doesn't emit any output */ => {
    //         execute_state_git();
    //     }
    //     Err(error) => log_diagnostic(
    //         DiagnosticKind::Error {
    //             subject: "failed to stage (git)",
    //             body: &format!("{}", error)
    //         }
    //     )
    // }
}

pub fn execute_branch_mercurial() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud branch (mercurial)",
    });
}

pub fn execute_branch_breezy() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud branch (breezy)",
    });
}
