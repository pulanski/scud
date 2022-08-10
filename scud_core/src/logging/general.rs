use std::{process::Command, time::SystemTime};

use colored::Colorize;

use crate::{
    cli::VCS,
    helpers::detect_vcs,
    logging::{
        diagnostics::{log_diagnostic, DiagnosticKind},
        helpers::{black_italic_quote, bright_yellow_backtick},
    },
};

pub fn log_execution_time(start_time: SystemTime) {
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();

    if duration.as_secs() > 0 {
        println!(
            "{}{}{}",
            "💥 Done in ".italic(),
            end_time
                .duration_since(start_time)
                .unwrap()
                .as_secs_f32()
                .to_string()
                .italic(),
            "s.".italic(),
        );
    } else {
        println!(
            "{}{}{}",
            "💥 Done in ".italic(),
            end_time
                .duration_since(start_time)
                .unwrap()
                .as_millis()
                .to_string()
                .italic(),
            "ms.".italic(),
        );
    }
}

pub fn log_dry_run_note() {
    log_diagnostic(
        DiagnosticKind::Note {
            body: &format!(
                "{} {}{}{} {}",
                "The".bright_yellow(),
                black_italic_quote(),
                "--dry-run".yellow().italic(),
                black_italic_quote(),
                "flag means no changes were made.".bright_yellow()
            ),
        }, // format!("{}{}{}{}{}\n",
           // "The ".bright_yellow(),
           // black_italic_quote(),
           // "--dry-run".yellow().italic(),
           // black_italic_quote(),
           // " flag means no changes were made.".bright_yellow()
           // )
    );
}

// TODO need to be okay with deleting this and refactoring
pub fn log_repo_status(_dry_run: bool) {
    let vcs = detect_vcs();

    match vcs {
        VCS::Breezy => {
            print!("{}", " Breezy ".yellow());
        }
        VCS::Git => {
            print!("{}", " Git ".bright_black().italic().on_bright_yellow());
        }
        VCS::Mercurial => {
            print!("{}", " Mercurial ".yellow());
        }
    }

    println!(
        "{}{}\n",
        " repository status".yellow(),
        " (up to date)".bright_yellow().italic(),
    );

    // let status_raw = Command::new("git")
    //     .arg("status")
    //     .arg("--short")
    //     .output()
    //     .expect("Failed to get git status").stdout;

    let status = Command::new("git").arg("status").arg("-sb").output();

    // let branch be first line read in

    match status {
        Ok(status) => {
            let status_raw = String::from_utf8_lossy(&status.stdout).to_string();

            // get tokens for status
            // branch information and changed files
            let mut status_tokens = status_raw.lines();

            // split branch info from changed files
            let _branch_info = status_tokens.next().unwrap();
            println!(
                "{}{}{}{}\n",
                "Branch info:  ".bright_blue(),
                " main ".black().on_green(),
                " ... ".black(),
                " origin/main ".black().on_red()
            );
            println!("{}\n", " CHANGES TO FILES: ".on_bright_green());

            // for file in status_tokens {
            //     let file_changes = file.split(" ").collect::<Vec<&str>>();
            //     let file_status = file_changes[0];
            //     print!("{}\t", file_status.to_string());
            //     println!("{}", file);
            // }
            // Changes staged for commit
            // git diff --staged --name-status
            // Changes not staged for commit

            // scud setup format prettier

            // TODO cap -> scud commit-all && scud push
            // TODO capr ->

            // Changes to be committed section
            println!("  {}\n", "Changes staged for commit: ".black().italic());
            println!(
                "    {}  {}",
                " M ".black().on_green(),
                "../../../Cargo.lock".italic().bright_green()
            );
            println!(
                "    {}  {}",
                " M ".black().on_green(),
                "../../../Cargo.toml".italic().bright_green()
            );

            // Changes not staged for commit
            println!(
                "\n  {}\n",
                "Changes not staged for commit: ".black().italic()
            );
            println!(
                "    {}  {}",
                " M ".black().on_red(),
                "commit/commit.rs".italic().bright_red()
            );
            println!(
                "    {}  {}",
                " M ".black().on_red(),
                "push/push.rs".italic().bright_red()
            );

            // Untracked files
            println!("\n  {}\n", "Untracked files: ".black().italic());
            println!(
                "    {}  {}",
                " ?? ".black().on_red(),
                "setup.rs".italic().bright_red()
            );
            println!(
                "    {}  {}\n",
                " ?? ".black().on_red(),
                "setup/".italic().bright_red()
            );

            log_diagnostic(DiagnosticKind::Tip {
                body: &format!(
                    "{} {}{}{} {} {}{}{} {}",
                    "Use".yellow(),
                    bright_yellow_backtick(),
                    "scud stage".green().italic(),
                    bright_yellow_backtick(),
                    "to stage all unstaged changes and untracked files for commit or use".yellow(),
                    bright_yellow_backtick(),
                    "scud commit".green().italic(),
                    bright_yellow_backtick(),
                    "to commit any staged changes".yellow()
                ),
            });

            // TODO
            // go through lines of status text
            // if first char is M
            // process_breezy_status
            // process_mercurial_status
        }
        Err(error) => {
            println!("{}", error);
            todo!("Needs to be worked on");
        }
    }
}
