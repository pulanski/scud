use colored::{
    Color,
    Colorize,
};

use crate::logging::helpers::{
    black_colon,
    black_comma,
    black_italic_close_paren,
    black_italic_implies,
    black_italic_open_paren,
    black_period,
    bright_yellow_backtick,
    bright_yellow_dots,
    yellow_backtick,
};

#[derive(Debug)]
pub enum DiagnosticKind<'a> {
    ScudCommandInfo {
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
    GeneralCommandInfo {
        command_name: &'a str, // e.g. info system => `scud info system`
        commands:     Vec<ExternalCommandInfo<'a>>, /* (command_name, link,
                                * description) */
    },
    WorkInProgress {
        feature: &'a str,
    },
}

#[derive(Debug, Clone)]
pub struct ExternalCommandInfo<'a> {
    pub command_name:        &'a str,
    pub command_link:        &'a str,
    pub command_description: &'a str,
}

// pub struct Command

pub fn log_diagnostic(diagnostic_kind: DiagnosticKind) {
    match diagnostic_kind {
        DiagnosticKind::ScudCommandInfo {
            command,
            description,
        } => {
            const VERSION: &str = env!("CARGO_PKG_VERSION");

            println!(
                "\n{}{}{}{} {}{}{}\n\n{}\n",
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
        DiagnosticKind::GeneralCommandInfo {
            command_name,
            commands,
        } => {
            // TODO
            // [ ] get the successful commit pipeline finished
            // [ ] get the current branch status (behind 1, ahead 1)
            //      when invoking state
            // [ ] get the branch listing command finished
            println!(
                "{}{}{}{}{}{}{}{}{}{}\n",
                " INFO ".black().on_yellow(),
                " Under the hood".bright_yellow().italic(),
                black_comma(),
                " the following commands are called when ".yellow().italic(),
                bright_yellow_backtick(),
                "scud ".green().italic(),
                command_name.green().italic(),
                bright_yellow_backtick(),
                " is invoked".yellow().italic(),
                black_colon(),
            );
            for general_command in commands {
                println!(
                    "{}{}{}{} {}{}{} {} {}{}{}\n",
                    "  *- ".bright_red().italic(),
                    bright_yellow_backtick(),
                    general_command.command_name.cyan().italic(),
                    bright_yellow_backtick(),
                    black_italic_open_paren(),
                    general_command.command_link.black().italic(),
                    black_italic_close_paren(),
                    bright_yellow_dots(),
                    "to ".yellow().italic(),
                    general_command.command_description.yellow().italic(),
                    black_period(),
                );
            }
            println!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
                " NOTE ".black().on_bright_yellow(),
                " If any of these commands are ".bright_yellow().italic(),
                "not installed ".red().italic(),
                "on your system".bright_yellow().italic(),
                black_comma(),
                " scud ".cyan().italic(),
                "will encounter a ".bright_yellow().italic(),
                "runtime error".red().italic(),
                " when ".bright_yellow().italic(),
                yellow_backtick(),
                "scud ".green().italic(),
                command_name.green().italic(),
                yellow_backtick(),
                " is invoked and will provide you with some "
                    .bright_yellow()
                    .italic(),
                "help getting up and running".yellow().italic(),
                " with the various dependencies that are the backbone for the many \
                 of the features of "
                    .bright_yellow()
                    .italic(),
                "scud".cyan().italic(),
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
                "{}{}{}{}{}{}{}{}{}{}{}\n\n{} {} {}\n\n{} {} {}\n\n{} {} {}\n",
                " INFO ".on_yellow(),
                " Detailed below are the ".yellow(),
                "underlying commands".blue().italic(),
                " issued for ".yellow(),
                "supported version control systems".bright_yellow().italic(),
                " when using ".yellow(),
                bright_yellow_backtick(),
                "scud ".green().italic(),
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
