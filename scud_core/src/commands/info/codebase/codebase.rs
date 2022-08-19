use std::{
    process::Command,
    time::SystemTime,
};

use colored::Colorize;

use crate::{
    cli::Codebase,
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    general::log_execution_time,
    logging::helpers::{
        black_period,
        bright_yellow_backtick,
    },
};

// TODO refactor this command to follow pattern of others (stage, state)
pub fn codebase_command(codebase_options: Codebase, start_time: SystemTime) {
    if codebase_options.info {
        println!(
            "{} {} {}{}{} {} {} {}{}{} {}{}\n",
            " INFO ".black().on_yellow(),
            "Under the hood, the".yellow().italic(),
            bright_yellow_backtick(),
            "onefetch".cyan().italic(),
            bright_yellow_backtick(),
            "(https://github.com/o2sh/onefetch)".black().italic(),
            "command is called when".yellow().italic(),
            bright_yellow_backtick(),
            "scud info codebase".green().italic(),
            bright_yellow_backtick(),
            "is invoked".yellow().italic(),
            black_period()
        );
    } else {
        log_diagnostic(DiagnosticKind::CommandInfo {
            command:     "codebase information",
            description: "This command is intended to display detailed information \
                          about the contents of the codebase within the current \
                          directory.",
        });

        match Command::new("onefetch").output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(_) => {
                log_diagnostic(DiagnosticKind::Error {
                    subject: &format!(
                        "onefetch {}",
                        "(https://github.com/o2sh/onefetch)".black().italic()
                    ),
                    body:    "is not installed on system locally or not found in \
                              PATH.",
                });
                log_diagnostic(DiagnosticKind::Hint {
                    body:    &format!(
                        "{}{}{}",
                        "If you wish to use scud's".yellow(),
                        "codebase information".bright_yellow().italic(),
                        "command , try installing it with scud's built-in setup \
                         command"
                            .yellow(),
                    ),
                    command: "scud setup info codebase",
                });
                // log_diagnostic(DiagnosticKind::Error {
                //     // TODO refactor
                //     subject: "codebase information (onefetch) TODO refactor
                // to add setup tip and setup command",
                //     body: &format!("{}", error),
                // });
            }
        }
    }

    log_execution_time(start_time);
}
