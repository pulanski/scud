use std::process::Command;

use colored::Colorize;

use crate::diagnostics::{
    log_diagnostic,
    DiagnosticKind,
};

pub fn execute_state_info() {
    log_diagnostic(DiagnosticKind::CommandInfo {
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
        "\n{} {}\n",
        " Git ".black().italic().on_bright_yellow(),
        "repository status".yellow().italic(),
    );

    // println!("\n{}\n", " BRANCH INFORMATION: ".black().on_cyan());

    // // Get current branch status
    // match Command::new("git").arg("status").arg("-sb").output() {
    //     Ok(status) => {
    //         let status_raw = String::from_utf8_lossy(&status.stdout).to_string();

    // if there are no unstaged and no untracked files, then the repository is clean
    // and issuing the stage command will have no effect on the repository
    // and that control flow will be output back to the user as a diagnostic

    //         // Get current branches
    //         let current_branches = status_raw
    //             .lines()
    //             .nth(0)
    //             .unwrap_or_default()
    //             .split(" ")
    //             .nth(1)
    //             .unwrap_or_default();

    //         // TODO create branching logs to look like
    //         //  /-
    //         //  |
    //         //  |*
    //         //  |
    //         //  \-

    //         // Get local branch
    //         let local_branch_raw =
    // current_branches.split("...").nth(0).unwrap_or_default();         let
    // local_branch = &format!(" {local_branch_raw} ");

    //         // Get remote branch
    //         let remote_branch_raw =
    // current_branches.split("...").nth(1).unwrap_or_default();         let
    // remote_branch = &format!(" {remote_branch_raw} ");

    //         println!(
    //             "{}{}  {} {} {}\n",
    //             "Current branch".bright_green().italic(),
    //             black_colon(),
    //             local_branch.to_string().black().italic().on_green(),
    //             "...".yellow().italic(),
    //             remote_branch.to_string().black().italic().on_red(),
    //         );
    //     }
    //     Err(error) => log_diagnostic(DiagnosticKind::Error {
    //         subject: "git status failed",
    //         body: &format!("{}", error),
    //     }),
    // }

    // // Get all branches
    // match Command::new("git").arg("branch").arg("-a").output() {
    //     Ok(branches) => {
    //         let branches_raw =
    // String::from_utf8_lossy(&branches.stdout).to_string();

    //         // Get all local branches
    //         let branches = branches_raw
    //             .lines()
    //             .filter(|line| !line.contains("remotes"))
    //             .map(|line| line.trim())
    //             .collect::<Vec<&str>>();

    //         println!(
    //             "{}{}\n",
    //             "Local branches".bright_yellow().italic(),
    //             black_colon(),
    //         );

    //         for branch_raw in branches {
    //             if branch_raw.contains("*") {
    //                 let branch = format!(" {} ", &branch_raw.replace("*",
    // "").trim().to_string());                 println!("{}\n",
    // branch.black().italic().on_yellow());             } else {
    //                 let branch = &format!(" {branch_raw} ");
    //                 println!("{}\n", branch.black().italic().on_yellow(),);
    //             }
    //         }

    //         println!("Potential Branch Metadata implementations\n\nDESIGN
    // ONE\n");

    //         println!(
    //             "{} {} {} {}\n{}",
    //             "/-".bright_black(),
    //             " fix/test ".bright_green().italic(),
    //             "...".black().italic(),
    //             " origin/fix/test ".bright_magenta().italic(),
    //             "|".bright_black()
    //         );
    //         println!(
    //             "{}{} {} {} {} {} {}",
    //             "|".bright_black(),
    //             "-*".yellow(),
    //             " main ".black().italic().on_green(),
    //             "---".yellow().italic(),
    //             " origin/main ".black().italic().on_red(),
    //             "<-".bright_black(),
    //             "ACTIVE".bright_cyan()
    //         );
    //         println!(
    //             "{} {} {} {}",
    //             "|\n|-".bright_black(),
    //             " feature/design_api ".bright_green().italic(),
    //             "...".black().italic(),
    //             " origin/feature/design_api ".bright_magenta().italic()
    //         );
    //         println!(
    //             "{} {} {} {}",
    //             "|\n\\-".bright_black(),
    //             " feature/state_ux ".bright_green().italic(),
    //             "...".black().italic(),
    //             " origin/fix/test ".bright_magenta().italic()
    //         );
    //     }
    //     Err(error) => log_diagnostic(DiagnosticKind::Error {
    //         subject: "git status failed",
    //         body: &format!("{}", error),
    //     }),
    // }

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
            // [behind 1]
            // let ahead = ahead_behind_split.next().unwrap();
            // let behind = ahead_behind_split.next().unwrap();
            // let ahead_int = ahead.parse::<i32>().unwrap();
            // let behind_int = behind.parse::<i32>().unwrap();
            // if ahead_int > 0 {
            //     println!(
            //         "  {}{}\n",
            //         "You are commits ahead of the
            // remote...".black().italic(),         ahead_int
            //     );
            // }
            // if behind_int > 0 {
            //     println!(
            //         "  {}{}\n",
            //         "You are commits behind the remote...".black().italic(),
            //         behind_int
            //     );
            // }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting status diff (git)",
            body:    &format!("{}", error),
        }),
    }
}

pub fn execute_state_mercurial() {}

pub fn execute_state_breezy() {}
