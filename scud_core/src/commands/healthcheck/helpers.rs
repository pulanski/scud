use std::process::Command;

use colored::Colorize;

use crate::diagnostics::{log_diagnostic, DiagnosticKind};

////////////////////////////////////
// Version Control System helpers //
////////////////////////////////////

pub fn healthcheck_vcs_git() {
    println!("{}", "Git:".bright_green());

    // TODO refactor to use log_diagnostic
    match Command::new("git").arg("--version").output() {
        Ok(output) => {
            let git_version = String::from_utf8_lossy(&output.stdout).to_string();
            print!("\n{}", git_version.black().italic());
        }
        Err(_) => {
            println!(
                "\n{}{}{}{}{}{}",
                " ERROR ".on_red(),
                " Git is not installed on system locally or not found in PATH.\n\n"
                    .bright_red()
                    .italic(),
                "Hint: ".green().bold(),
                "Try installing it with scud's built-in setup command `".yellow(),
                "scud setup vcs git".italic().green(),
                "`.".yellow()
            );
        }
    }
}

pub fn healthcheck_vcs_hg() {
    println!("\n{}", "Mercurial:".bright_green());

    // TODO refactor to use log_diagnostic

    match Command::new("hg").arg("--version").output() {
        Ok(output) => {
            let hg_version = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "\n{}",
                hg_version.split("\n").next().unwrap().italic().black()
            );
        }
        Err(_) => {
            println!("\n{}{}{}{}{}{}", " ERROR ".on_red(), " Mercurial is not installed on system locally or not found in PATH.\n\n".bright_red().italic(), "Hint: ".green().italic(), "If you wish to use Mercurial for your underlying VCS with scud, try installing it on your local machine with scud's builtin setup command `".yellow(), "scud setup vcs mercurial".italic().green(), "`.".yellow());
        }
    }
}

pub fn healthcheck_vcs_brz() {
    println!("\n{}", "Breezy:".bright_green());

    // TODO refactor to use log_diagnostic

    match Command::new("brz").arg("--version").output() {
        Ok(output) => {
            let brz_version = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "\n{}",
                brz_version.split("\n").next().unwrap().italic().black()
            );
        }
        Err(_) => {
            println!(
                "\n{}{}{}{}{}{}",
                " ERROR ".on_red(),
                " Breezy is not installed on system locally or not found in PATH.\n\n"
                    .bright_red()
                    .italic(),
                "Hint: ".bright_green(),
                "Try installing it with scud's builtin setup command `".yellow(),
                "scud setup vcs breezy".italic().green(),
                "`.".yellow()
            );
        }
    }
}

////////////////////////////
// Source Control helpers //
////////////////////////////

pub fn healthcheck_sc_github() {
    println!("\n{}", "GitHub:".bright_green());

    match Command::new("gh").arg("--version").output() {
        Ok(output) => {
            let github_version = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "\n{}",
                github_version.split("\n").next().unwrap().italic().black()
            );
        }
        Err(_) => {
            log_diagnostic(DiagnosticKind::Error {
                subject: "Github CLI",
                body: "is not installed on system locally or not found in PATH.",
            });

            log_diagnostic(
                    DiagnosticKind::Hint {
                        body: &format!(
                            "{}{}{}",
                            "If you wish to use "
                                .yellow(),
                            "GitHub"
                                .bright_yellow()
                                .italic(),
                            " for your underlying source control provider within scud, try installing it with scud's built-in setup command "
                                .yellow(),
                        ),
                        command: "scud setup sc github"
                    }
                );
        }
    }
}

pub fn healthcheck_sc_gitlab() {
    println!("\n{}", "GitLab:".green().bold());

    match Command::new("glab").arg("--version").output() {
        Ok(output) => {
            let gitlab_version = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "\n{}\n",
                gitlab_version.split("\n").next().unwrap().italic().black()
            );
        }
        Err(_) => {
            log_diagnostic(DiagnosticKind::Error {
                subject: "GitLab CLI",
                body: "is not installed on system locally or not found in PATH.",
            });
            log_diagnostic(
                    DiagnosticKind::Hint {
                        body: &format!(
                            "{}{}{}",
                            "If you wish to use "
                                .yellow(),
                            "GitLab"
                                .bright_yellow()
                                .italic(),
                            " for your underlying source control provider within scud, try installing it with scud's built-in setup command"
                                .yellow(),
                        ),
                        command: "scud setup sc gitlab"
                    }
                );
        }
    }
}
