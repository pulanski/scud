use std::process::Command;

use crate::{
    commands::commit::helpers::{process_commit_message,check_for_staged_files},
    diagnostics::{log_diagnostic, DiagnosticKind},
};

//////////////////////////////////////////////////////////
// Functions handle the execution of each the supported //
// underlying VCS implementations of the commit command //
//////////////////////////////////////////////////////////

pub fn execute_commit_git() {
    check_for_staged_files();

    let commit_message = process_commit_message();

    match Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
    {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "git commit failed",
            body: &format!("{}", error),
        }),
    }
}

pub fn execute_commit_mercurial() {
    let commit_message = process_commit_message();

    todo!("execute_commit_mercurial");
}

pub fn execute_commit_breezy() {
    let commit_message = process_commit_message();

    todo!("execute_commit_breezy");
}
