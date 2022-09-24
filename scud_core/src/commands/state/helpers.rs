use std::process::Command;

use colored::Colorize;

use crate::diagnostics::{log_diagnostic, DiagnosticKind};

pub fn get_commit_diff(status_diff: String) -> String {
    let mut status = status_diff.split("\n");

    // (i.e. "## master...origin/master [ahead 1]")
    let branch_metadata = status.next().unwrap();

    // split on the space if the number of tokens is 2
    //

    let branch_tokens = branch_metadata.split(" ").collect::<Vec<&str>>();

    if branch_tokens.len() > 2 {
        let branch_metadata_tokens =
            branch_metadata.split('[').collect::<Vec<&str>>();

        let ahead_behind_raw = branch_metadata_tokens.last().unwrap();
        return ahead_behind_raw.split(']').next().unwrap().to_string();
    } else {
        return String::new();
    }
}

pub fn display_commit_state(ahead_behind: String) {
    // parse the ahead and behind numbers
    // if behind > 0, print warning
    // if ahead > 0, print info

    if ahead_behind.is_empty() {
        return;
    }

    if ahead_behind.contains(",") {
        // parse the ahead and behind numbers
        let ahead_behind = ahead_behind
            .split(",")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let ahead = ahead_behind[0].parse::<i32>().unwrap();
        let behind = ahead_behind[1].parse::<i32>().unwrap();

        println!(
            "{}{}{}{}{}{}{}{}\n",
            "Your are ".magenta().italic(),
            "behind ".bright_red().italic(),
            "the remote repository by ".magenta().italic(),
            format!("{} commits", behind).bright_red().italic(),
            "and ".magenta().italic(),
            "ahead ".bright_green().italic(),
            "by ".magenta().italic(),
            format!("{} commits", ahead).bright_green().italic()
        );
    } else {
        let ahead_or_behind = ahead_behind
            .split(" ")
            .map(|s| s.trim())
            .collect::<Vec<&str>>()[0];

        match ahead_or_behind {
            "ahead" => {
                let ahead = ahead_behind
                    .split(" ")
                    .map(|s| s.trim())
                    .collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap();

                println!(
                    "{}{}{}{}{}\n",
                    " INFO ".black().on_bright_yellow(),
                    " You are ".magenta().italic(),
                    "ahead ".bright_green().italic(),
                    "of the remote repository by ".magenta().italic(),
                    format!("{} commit(s)", ahead).bright_green().italic(),
                );
            }
            "behind" => {
                let behind = ahead_behind
                    .split(" ")
                    .map(|s| s.trim())
                    .collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap();

                println!(
                    "{}{}{}{}{}\n",
                    " WARNING ".black().on_bright_yellow(),
                    " You are ".magenta().italic(),
                    "behind ".bright_red().italic(),
                    "the remote repository by ".magenta().italic(),
                    format!("{} commit(s)", behind).bright_red().italic(),
                );
            }
            _ => unreachable!(),
        }
    }
}

pub fn display_all_staged_changes() {
    println!("{}\n", " STAGED CHANGES: ".black().on_cyan());
    let filters = vec!["M", "D", "R"];

    let mut any_changes = false;

    // check for any staged changes
    match Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
    {
        Ok(output) => {
            let staged_changes = String::from_utf8_lossy(&output.stdout).to_string();
            if !staged_changes.is_empty() {
                any_changes = true;
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting staged changes (git)",
            body: &format!("{}", error),
        }),
    }

    if any_changes {
        // display different types of staged changes
        // (modified, deleted, renamed) to the user
        for filter in filters {
            match Command::new("git")
                .args(["diff", "--name-only", "--staged", "--diff-filter", filter])
                .output()
            {
                Ok(output) => {
                    let staged_changes =
                        String::from_utf8_lossy(&output.stdout).to_string();

                    match filter {
                        "M" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "modified files ".bright_yellow().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Modified files ".bright_yellow().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " M ".black().on_bright_yellow(),
                                        staged_change
                                            .to_string()
                                            .bright_yellow()
                                            .italic()
                                    );
                                }
                                println!()
                            }
                        }
                        "D" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "deleted files ".red().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Deleted files ".red().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " D ".black().on_bright_red(),
                                        staged_change
                                            .to_string()
                                            .bright_red()
                                            .italic()
                                    );
                                }
                            }
                        }
                        "R" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "renamed files ".bright_blue().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Renamed files ".bright_blue().italic(),
                                    "staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " R ".black().on_bright_blue(),
                                        staged_change
                                            .to_string()
                                            .bright_blue()
                                            .italic()
                                    );
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Err(error) => log_diagnostic(DiagnosticKind::Error {
                    subject: "getting staged changes (git)",
                    body: &format!("{}", error),
                }),
            }
        }
    } else {
        println!(
            "  {}{}{}{}\n",
            "No ".magenta().italic(),
            "staged changes ".cyan().italic(),
            "to commit ".magenta().italic(),
            "...".black().italic()
        );
    }
}

pub fn display_all_unstaged_changes() {
    println!("{}\n", " UNSTAGED CHANGES: ".black().on_cyan());
    let filters = vec!["M", "D", "R"];

    let mut any_changes = false;

    // check for any unstaged changes
    match Command::new("git").args(["diff", "--name-only"]).output() {
        Ok(output) => {
            let unstaged_changes =
                String::from_utf8_lossy(&output.stdout).to_string();
            if !unstaged_changes.is_empty() {
                any_changes = true;
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting staged changes (git)",
            body: &format!("{}", error),
        }),
    }

    if any_changes {
        // display different types of staged changes
        // (modified, deleted, renamed) to the user
        for filter in filters {
            match Command::new("git")
                .args(["diff", "--name-only", "--diff-filter", filter])
                .output()
            {
                Ok(output) => {
                    let staged_changes =
                        String::from_utf8_lossy(&output.stdout).to_string();

                    match filter {
                        "M" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "modified files ".bright_yellow().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Modified files ".bright_yellow().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " M ".black().on_bright_yellow(),
                                        staged_change
                                            .to_string()
                                            .bright_yellow()
                                            .italic()
                                    );
                                }
                                println!()
                            }
                        }
                        "D" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "deleted files ".red().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Deleted files ".red().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " D ".black().on_bright_red(),
                                        staged_change
                                            .to_string()
                                            .bright_red()
                                            .italic()
                                    );
                                }
                            }
                        }
                        "R" => {
                            if staged_changes.is_empty() {
                                println!(
                                    "  {}{}{}{}\n",
                                    "No ".magenta().italic(),
                                    "renamed files ".bright_blue().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic()
                                );
                            } else {
                                println!(
                                    "  {}{}{}\n",
                                    "Renamed files ".bright_blue().italic(),
                                    "not yet staged for commit ".magenta().italic(),
                                    "...".black().italic(),
                                );
                                for staged_change in staged_changes.lines() {
                                    println!(
                                        "    {}  {}",
                                        " R ".black().on_bright_blue(),
                                        staged_change
                                            .to_string()
                                            .bright_blue()
                                            .italic()
                                    );
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Err(error) => log_diagnostic(DiagnosticKind::Error {
                    subject: "getting staged changes (git)",
                    body: &format!("{}", error),
                }),
            }
        }
    } else {
        println!(
            "  {}{}{}{}\n",
            "No ".magenta().italic(),
            "unstaged changes ".cyan().italic(),
            "to commit ".magenta().italic(),
            "...".black().italic()
        );
    }
}

pub fn display_state_header() {
    println!(
        "\n{} {}",
        " Git ".black().italic().on_bright_yellow(),
        "repository status".yellow().italic(),
    );
}

pub fn display_all_untracked_changes() {
    println!("{}\n", " UNTRACKED CHANGES: ".black().on_cyan());

    // check for any untracked changes
    match Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .output()
    {
        Ok(output) => {
            let untracked_changes =
                String::from_utf8_lossy(&output.stdout).to_string();
            if !untracked_changes.is_empty() {
                println!(
                    "  {}{}{}\n",
                    "New files ".yellow().italic(),
                    "not included in the previous commit ".magenta().italic(),
                    "...".black().italic(),
                );
                for untracked_file in untracked_changes.lines() {
                    println!(
                        "    {}  {}",
                        " ?? ".black().on_yellow(),
                        untracked_file.to_string().yellow().italic()
                    );
                }
                println!();
            } else {
                println!(
                    "  {}{}{}{}\n",
                    "No ".magenta().italic(),
                    "untracked changes ".cyan().italic(),
                    "to commit ".magenta().italic(),
                    "...".black().italic()
                );
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting staged changes (git)",
            body: &format!("{}", error),
        }),
    }
}

pub fn display_current_branch() {
    match Command::new("git")
        .args(["branch", "--show-current"])
        .output()
    {
        Ok(output) => {
            let current_branch = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "\n{}{}{}{}\n",
                "On branch: ".black().italic(),
                " ".on_blue(),
                current_branch.trim().black().italic().on_blue(),
                " ".on_blue()
            );
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting current branch (git)",
            body: &format!("{}", error),
        }),
    }
}
