use std::time::SystemTime;

use crate::{
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    general::log_execution_time,
    information::System,
};

pub fn system_command(system_options: System, start_time: SystemTime) {
    if system_options.info {
        println!("System: info");
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
