use colored::{
    Color,
    Colorize,
};

use crate::logging::helpers::{
    black_colon,
    black_comma,
    black_italic_implies,
    black_period,
    bright_yellow_backtick,
    yellow_backtick,
};

#[derive(Debug)]
pub enum DiagnosticKind<'a> {
    CommandInfo {
        command:     &'a str,
        description: &'a str,
    },
    DryRun {
        command: &'a str,
    },
    Error {
        subject: &'a str,
        body:    &'a str,
    },
    Hint {
        body:    &'a str,
        // TODO refactor body to
        // service: &'a str,
        // provider: &'a str,
        command: &'a str,
    },
    Info,
    Note {
        body: &'a str,
    },
    Tip {
        body: &'a str,
    },
    VCSInfo {
        command_name:      &'a str,
        git_command:       &'a str,
        mercurial_command: &'a str,
        breezy_command:    &'a str,
    },
    WorkInProgress {
        feature: &'a str,
    },
}

pub fn log_diagnostic(diagnostic_kind: DiagnosticKind) {
    match diagnostic_kind {
        DiagnosticKind::CommandInfo {
            command,
            description,
        } => {
            const VERSION: &str = env!("CARGO_PKG_VERSION");

            println!(
                "{}{}{}{} {}{}{}\n\n{}\n",
                " Scud ".black().on_yellow(),
                "v".black().italic().on_yellow(),
                VERSION.black().italic().on_yellow(),
                " ".black().italic().on_yellow(),
                " ".on_bright_red(),
                command.to_string().to_uppercase().black().on_bright_red(),
                " COMMAND ".black().on_bright_red(),
                description.to_string().italic()
            );
        }
        DiagnosticKind::DryRun { command } => {
            println!(
                "\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
                " DRY RUN ".on_color(Color::TrueColor {
                    r: 239,
                    g: 175,
                    b: 3,
                }),
                "  No ".bright_yellow().italic(),
                command.cyan().italic(),
                " was executed".bright_yellow().italic(),
                black_period(),
                " To ".bright_yellow().italic(),
                "properly ".bright_yellow().italic(),
                command.bright_yellow().italic(),
                black_comma(),
                " rerun".yellow().italic(),
                " the ".bright_yellow().italic(),
                yellow_backtick(),
                "scud ".green(),
                command.green(),
                yellow_backtick(),
                " command ".bright_yellow().italic(),
                "without".yellow().italic(),
                " the ".bright_yellow().italic(),
                "\"".black().italic(),
                "--".bright_yellow().italic(),
                "dry-run".yellow().italic(),
                "\"".black().italic(),
                " flag".bright_yellow().italic(),
                black_period(),
            );
        }
        DiagnosticKind::Error { subject, body } => {
            println!(
                "\n{} {}\n\n{}\n\n{}{}\n",
                " ERROR ".black().on_red(),
                subject.to_string().red(),
                body.to_string().italic(),
                "For more information, please see the scud documentation at "
                    .bright_yellow(),
                "https://scud.dev/docs/".bright_cyan().italic()
            );
        }
        DiagnosticKind::Hint { body, command } => {
            println!(
                "{}{} {} {}{}{}{}\n",
                "Hint".green().italic(),
                black_colon(),
                body.to_string(),
                bright_yellow_backtick(),
                command.to_string().green().italic(),
                bright_yellow_backtick(),
                black_period()
            );
        }
        DiagnosticKind::Info => {
            println!("{}", " INFO ".on_yellow());
        }
        DiagnosticKind::Note { body } => {
            println!("{} {}\n", " NOTE ".on_bright_yellow(), body.to_string());
        }
        DiagnosticKind::Tip { body } => {
            println!(
                "{}{} {}{}\n",
                "Tip".green().italic(),
                black_colon(),
                body.to_string(),
                black_period()
            );
        }
        DiagnosticKind::VCSInfo {
            command_name,
            git_command,
            mercurial_command,
            breezy_command,
        } => {
            println!(
                "{} {} {}{} {}{}{}\n\n{} {} {}\n\n{} {} {}\n\n{} {} {}\n",
                " INFO ".on_yellow(),
                "Underlying commands issued for supported version control systems \
                 during"
                    .yellow()
                    .italic(),
                bright_yellow_backtick(),
                "scud".green().italic(),
                command_name.to_string().green().italic(),
                bright_yellow_backtick(),
                black_period(),
                " Git ".bright_black().italic().on_bright_yellow(),
                black_italic_implies(),
                git_command.to_string().bright_cyan().italic(),
                " Mercurial ".bright_black().italic().on_bright_yellow(),
                black_italic_implies(),
                mercurial_command.to_string().bright_cyan().italic(),
                " Breezy ".bright_black().italic().on_bright_yellow(),
                black_italic_implies(),
                breezy_command.to_string().bright_cyan().italic(),
            );
        }
        DiagnosticKind::WorkInProgress { feature } => {
            println!(
                "{} {} {}\n",
                " WORK IN PROGRESS ".on_bright_green(),
                feature.to_string().green().italic(),
                "is currently under development. What you're seeing is only its \
                 initial scaffolding"
                    .bright_green()
                    .italic()
            );
        }
    }
}

// TODO log at the end of status
// behind 1
// tip ...

// pub fn log_vcs_under_the_hood(git_command: &str, mercurial_command: &str,
// breezy_command: &str) {     println!("Git -> {}", git_command);
//     println!("Mercurial -> {}", mercurial_command);
//     println!("Breezy -> {}", breezy_command);
// }
