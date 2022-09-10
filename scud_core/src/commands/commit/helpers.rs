use std::process::{exit, Command};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use indicatif::ProgressBar;

use crate::{
    commands::commit::{commit::CommitMessageFormat, logging::log_commit_message},
    diagnostics::{log_diagnostic, DiagnosticKind},
    logging::helpers::bright_yellow_backtick,
};

/// Loop that asks the user for a commit message format, then asks the
/// user to confirm the commit message generated via the selection w/ the
/// cli.
///
/// Returns:
///
/// A String
pub fn process_commit_message() -> String {
    #[allow(unused_assignments)]
    let mut commit_message = String::new();

    loop {
        // TODO check for global config file and if so, skip this step
        let commit_message_format = get_commit_message_format();

        commit_message = String::new();

        match commit_message_format {
            CommitMessageFormat::Conventional => {
                commit_message = commit_conventional_standard();
            }
            CommitMessageFormat::Angular => {
                commit_message = commit_angular_standard();
            }
            CommitMessageFormat::None => {
                commit_message = commit_standard_none();
            }
            CommitMessageFormat::Unknown => {
                log_diagnostic(DiagnosticKind::Error {
                    subject: "Invalid commit message format",
                    body: "Unknown selection for commit message format. Valid \
                              formats include Conventional Commit Specification, \
                              Angular Commit Specification, or none",
                });
            }
        }

        let confirm_commit = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "{}{}",
                "Is this commit message ".bright_yellow().italic(),
                "correct".green().italic()
            ))
            .default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap();

        if confirm_commit {
            break;
        }

        clearscreen::clear().expect("failed to clear screen");
    }

    commit_message
}

// TODO refactor to commit/formatters.rs

/////////////////////////////////////////////////////
// Functions for processing user input in order to //
// generate a formatted commit message             //
//   (either following an established standard     //
//    or a custom message)                         //
/////////////////////////////////////////////////////

/// It asks the user for a commit type, scope, subject, body, breaking changes,
/// and referenced issues, then logs a prettified version of the commit message
/// to the user before returning it
///
/// Returns:
///
/// A String
pub fn commit_conventional_standard() -> String {
    let pb = ProgressBar::new(6);

    pb.inc(1);
    pb.message();

    let commit_type = get_commit_type();

    pb.inc(1);

    let scope = get_scope();

    pb.inc(1);

    let subject = get_subject(&commit_type, &scope);

    pb.inc(1);

    let body = get_body();

    pb.inc(1);

    let breaking_changes = get_breaking_changes();

    pb.inc(1);

    let referenced_issues = get_referenced_issues();

    pb.finish_and_clear();

    #[allow(unused_assignments)]
    let mut commit_message = String::new();

    commit_message = match scope.len() {
        0 => format!("{commit_type}: {subject}\n\n{body}"),
        _ => format!("{commit_type}({scope}): {subject}\n\n{body}"),
    };

    commit_message = match breaking_changes.len() {
        0 => commit_message,
        _ => format!("{commit_message}\n\nBREAKING CHANGE: {breaking_changes}"),
    };

    commit_message = match referenced_issues.len() {
        0 => commit_message,
        _ => format!("{commit_message}\n\nRefs: {referenced_issues}"),
    };

    log_commit_message(
        commit_type,
        scope,
        subject,
        body,
        breaking_changes,
        referenced_issues,
    );

    commit_message
}

pub fn commit_angular_standard() -> String {
    "angular message".to_string()
}

pub fn commit_standard_none() -> String {
    "none message".to_string()
}

/////////////////////////////////////////////////////
// Various functions for getting user input        //
// in the process of generating the commit message //
/////////////////////////////////////////////////////

/// It asks the user to select a commit message format, and returns the selected
/// format
///
/// Returns:
///
/// A CommitMessageFormat enum
pub fn get_commit_message_format() -> CommitMessageFormat {
    let commit_message_formatting_options = &[
        format!(
            "{}{}",
            "Conventional Commit Standard",
            "  (https://www.conventionalcommits.org/en/v1.0.0/)"
        ),
        format!(
            "{}{}",
            "Angular Commit Standard",
            "  (https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit)"
        ),
        format!(
            "{}{}",
            "None", "  (Simple commit message without formatting)"
        ),
    ];

    let selected_commit_message_format =
        FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "{}{}",
                "Select a ".bright_yellow().italic(),
                "commit message format".yellow().italic()
            ))
            .default(0)
            .items(&commit_message_formatting_options[..])
            .interact()
            .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    match selected_commit_message_format {
        0 => CommitMessageFormat::Conventional,
        1 => CommitMessageFormat::Angular,
        2 => CommitMessageFormat::None,
        _ => {
            log_diagnostic(DiagnosticKind::Error {
                subject: "Invalid commit message format",
                body: "Unknown selection for commit message format. Valid \
                          formats include Conventional Commit Specification, \
                          Angular Commit Specification, or none",
            });
            CommitMessageFormat::None
        }
    }
}

/// It returns the number of characters that can be used in the subject line of
/// a commit message
///
/// Arguments:
///
/// * `commit_type`: The type of commit, e.g. feat, fix, etc.
/// * `scope`: The scope of the commit.
///
/// Returns:
///
/// The remaining subject length.
pub fn get_remaining_subject_length(commit_type: &str, scope: &str) -> usize {
    let max_subject_length = 100;

    let mut remaining_subject_length =
        max_subject_length - scope.len() - commit_type.len() - 2;

    if scope.len() > 0 {
        remaining_subject_length -= 2;
    }

    remaining_subject_length
}

/// It takes a list of commit types, prompts the user to select one, and returns
/// the selected commit type
///
/// Returns:
///
/// A string
pub fn get_commit_type() -> String {
    let commit_type_options = &[
        "feat:  A new feature",
        "fix:  A bug fix",
        "docs:  Documentation only changes",
        "style:  Changes that do not affect the meaning of the code (white-space, \
         formatting, missing semi-colons, etc)",
        "refactor:  A code change that neither fixes a bug nor adds a feature",
        "perf:  A code change that improves performance",
        "test:  Adding missing tests or correcting existing tests",
        "build:  Changes that affect the build system or external dependencies \
         (example scons, gulp, grunt, broccoli, npm, etc.)",
        "ci:  Changes to our CI configuration files and scripts (example scopes: \
         Travis, Circle, BrowserStack, SauceLabs, etc.)",
        "chore:  Other changes that don't modify src/bin files",
        "revert:  Reverts a previous commit",
    ];

    // TODO log_commit_config(commit_config)
    // on each clearscreen::clear()
    // Partially generated commit message formatted following the Conventional
    // Commit Standard Generated commit message following the Angular Commit
    // Standard Generated commit message without formatting

    let selected_commit_type = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}{}{}",
            "Select the ".bright_yellow().italic(),
            "type of change".yellow().italic(),
            " that you're committing:".bright_yellow().italic()
        ))
        .default(0)
        .items(&commit_type_options[..])
        .interact()
        .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    commit_type_options[selected_commit_type]
        .split(":")
        .next()
        .unwrap()
        .to_string()
}

/// It asks the user for the scope of the change, and returns the answer
///
/// Returns:
///
/// A string
pub fn get_scope() -> String {
    let scope = Input::new()
        .with_prompt(format!(
            "\n{}{}{}{}:{}",
            "What is the ".bright_yellow().italic(),
            "scope".yellow().italic(),
            " of this change ".bright_yellow().italic(),
            "(e.g. component or file name)".green().italic(),
            " (press enter to skip)".black().italic()
        ))
        .default("".to_string())
        .interact_text()
        .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    scope
}

pub fn get_subject(commit_type: &str, scope: &str) -> String {
    let remaining_subject_length =
        get_remaining_subject_length(&commit_type, &scope);

    let subject: String = Input::new()
        .with_prompt(format!(
            "\n{} {}{}{} {} {}{}{}",
            "Write a".bright_yellow().italic(),
            "short".yellow().italic(),
            ",".black().italic(),
            " imperative tense description".yellow().italic(),
            "of the change".bright_yellow().italic(),
            "(max ".black().italic(),
            remaining_subject_length.to_string().black().italic(),
            " chars)".black().italic()
        ))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() <= remaining_subject_length {
                Ok(())
            } else {
                Err(
                    // TODO: look into refactoring using thiserror
                    "Provided subject exceeds character limit, please specify a \
                     subject with less than the maximum character limit",
                )
            }
        })
        .default("".into())
        .interact_text()
        .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    subject
}

pub fn get_body() -> String {
    let body = Input::new()
        .with_prompt(format!(
            "\n{}{}{}{}",
            "Provide a ".bright_yellow().italic(),
            "longer description".yellow().italic(),
            " of the change ".bright_yellow().italic(),
            " (press enter to skip)".black().italic()
        ))
        .default("".into())
        .interact_text()
        .unwrap();

    clearscreen::clear().expect("failed to clear screen");

    body
}

pub fn get_breaking_changes() -> String {
    // Continue

    let breaking_changes = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}{}{}",
            "Are there any ".bright_yellow().italic(),
            "breaking changes".yellow().italic(),
            "?".bright_yellow().italic()
        ))
        .default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();

    let mut breaking_changes_section = String::new();

    if breaking_changes {
        let breaking_changes_message: String = Input::new()
            .with_prompt(format!(
                "{}{}{}",
                "Provide".bright_yellow().italic(),
                " a description".yellow().italic(),
                " of the breaking changes".bright_yellow().italic()
            ))
            .default("".into())
            .interact_text()
            .unwrap();

        breaking_changes_section = format!("{}", breaking_changes_message)
    }

    clearscreen::clear().expect("failed to clear screen");

    breaking_changes_section
}

pub fn get_referenced_issues() -> String {
    let referenced_issues = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}{}{}",
            "Does this change ".bright_yellow().italic(),
            "affect any open issues".yellow().italic(),
            "?".bright_yellow().italic()
        ))
        .default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();

    let mut referenced_issues_section = String::new();

    if referenced_issues {
        let referenced_issues_message: String = Input::new()
            .with_prompt(format!(
                "{}{}{}",
                "Provide a comma separated ".bright_yellow().italic(),
                "list of issue numbers ".yellow().italic(),
                "(e.g. #123, #456, #789, etc.)".green().italic()
            ))
            .default("".into())
            .interact_text()
            .unwrap();

        referenced_issues_section = format!("{}", referenced_issues_message)
    }

    clearscreen::clear().expect("failed to clear screen");

    referenced_issues_section
}

pub fn check_for_staged_files() {
    // Get changes staged for commit
    // git diff --name-only --cached
    match Command::new("git")
        .args(["diff", "--name-only", "--cached"])
        .output()
    {
        Ok(output) => {
            let staged_changes = String::from_utf8_lossy(&output.stdout).to_string();

            if staged_changes.len() == 0 {
                log_diagnostic(DiagnosticKind::Error {
                    subject: "Attempting to commit without any staged changes",
                    body: "Please stage your changes before going through the \
                              commit process",
                });
                log_diagnostic(DiagnosticKind::Tip {
                    body: &format!(
                        "{} {}{}{} {}",
                        "Use".yellow(),
                        bright_yellow_backtick(),
                        "scud stage".green().italic(),
                        bright_yellow_backtick(),
                        "to stage all unstaged changes and untracked files for \
                         commit"
                            .yellow(),
                    ),
                });
                exit(1);
            }
        }
        Err(error) => log_diagnostic(DiagnosticKind::Error {
            subject: "getting staged changes (git)",
            body: &format!("{}", error),
        }),
    }
}
