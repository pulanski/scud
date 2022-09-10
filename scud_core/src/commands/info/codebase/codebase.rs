use std::{process::Command, time::SystemTime};

use colored::Colorize;

use crate::{
    diagnostics::{log_diagnostic, DiagnosticKind, ExternalCommandInfo},
    general::log_execution_time,
    information::Codebase,
};

// TODO refactor this command to follow pattern of others (stage, state)
pub fn info_codebase_command(codebase_options: Codebase, start_time: SystemTime) {
    if codebase_options.info {
        log_diagnostic(DiagnosticKind::ScudCommandInfo {
            command: "codebase information",
            description: "This command is intended to display detailed information \
                          about the contents of the codebase within the current \
                          directory [alias: cb]
                          ",
        });
        log_diagnostic(DiagnosticKind::GeneralCommandInfo {
            command_name: "codebase",
            commands: [ExternalCommandInfo {
                command_name: "info codebase",
                command_link: "https://github.com/o2sh/onefetch",
                command_description: "display detailed information about the \
                                      contents of the codebase within the current \
                                      directory.",
            }]
            .to_vec(),
        });
    } else {
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
                    body: "is not installed on system locally or not found in \
                              PATH.",
                });
                log_diagnostic(DiagnosticKind::Hint {
                    body: &format!(
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
