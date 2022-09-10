use colored::Colorize;

use crate::logging::helpers::{black_italic_close_paren, black_italic_open_paren};

// supports generating commit messages following an
// assortment of commit message formats (Angular, Conventional, etc.)
// for producing standardized, human-readable commit messages
// in a modular, and easily configurable manner
//
// TODO setup ignore
// setup ignore git
// setup ignore docker
// setup ignore
// setup ignore docker-machine

// TODO setup formatting
// setup formatting
// Select formatting tool for increased code quality via maintained consistent
// coding styles for multiple developers
//
// setup formatting editorconfig
// setup formatting prettier
// setup formatting --all
//
// help implement DRY patterns into your development workflow so you can focus
// on the core functionality of your project and the problems you are trying to
// solve

// setup quality trunk
// keeping things DRY
// in most projects you'll want some sort of ci/cd pipeline
// and instead of

pub fn log_commit_message(
    commit_type: String,
    scope: String,
    subject: String,
    body: String,
    breaking_changes: String,
    refs: String,
) {
    let commit_message_header = " GENERATED COMMIT MESSAGE ".on_red();
    let commit_message_note = "(colored output omitted from commit message)"
        .green()
        .italic();
    let commit_type = commit_type.bright_yellow();
    let left_paren = black_italic_open_paren();
    let scope = scope.green();
    let right_paren = black_italic_close_paren();
    let subject = subject.bright_yellow();
    let body = body.italic();
    let breaking_changes_header = "BREAKING CHANGE: ".red();
    let breaking_changes = breaking_changes.black().italic();
    let refs_header = "Refs: ".yellow();
    let refs = refs.black().italic();

    let mut logged_commit_message =
        format!("{commit_message_header} {commit_message_note}\n\n");

    logged_commit_message = match scope.len() {
        0 => format!("{logged_commit_message}{commit_type}: {subject}\n\n{body}"),
        _ => format!(
            "{logged_commit_message}{commit_type}{left_paren}{scope}{right_paren}: \
             {subject}\n\n{body}"
        ),
    };

    logged_commit_message = match breaking_changes.len() {
        0 => logged_commit_message,
        _ => format!(
            "{logged_commit_message}\n\n{breaking_changes_header}{breaking_changes}"
        ),
    };

    logged_commit_message = match refs.len() {
        0 => logged_commit_message,
        _ => format!("{logged_commit_message}\n\n{refs_header}{refs}"),
    };

    println!("{}\n", logged_commit_message);
}
