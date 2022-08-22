use std::time::SystemTime;

use crate::{
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
        ExternalCommandInfo,
    },
    general::log_execution_time,
    information::System,
};

pub fn info_system_command(system_options: System, start_time: SystemTime) {
    if system_options.info {
        execute_system_info();
    } else {
        execute_system();
    }

    log_execution_time(start_time);
}

pub fn execute_system() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "System info",
    });
}

pub fn execute_system_info() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command:     "system information",
        description: "This command is intended to display detailed information \
                      about the system on which scud is running [alias: sys]",
    });
    log_diagnostic(DiagnosticKind::GeneralCommandInfo {
        command_name: "info system",
        commands:     vec![
            ExternalCommandInfo {
                command_name:        "neofetch",
                command_link:        "https://github.com/dylanaraps/neofetch",
                command_description: "get system information",
            },
            ExternalCommandInfo {
                command_name:        "cpufetch",
                command_link:        "https://github.com/Dr-Noob/cpufetch",
                command_description: "get CPU architecture information",
            },
        ],
    })
}
