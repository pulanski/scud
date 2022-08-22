use std::time::SystemTime;

use colored::Colorize;

use crate::{
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    general::log_execution_time,
    information::System,
    logging::helpers::{
        black_colon,
        black_comma,
        black_period,
        bright_yellow_backtick,
        bright_yellow_dots,
    },
};

pub fn info_system_command(system_options: System, start_time: SystemTime) {
    if system_options.info {
        println!(
            "\n{}{}{}{}{}{}{}{}{}\n\n{}{}{}{}{}{}{}{}\n\n{}{}{}{}{}{}{}{}\n",
            " INFO ".black().on_yellow(),
            " Under the hood".yellow().italic(),
            black_comma(),
            " the following commands are called when ".yellow().italic(),
            bright_yellow_backtick(),
            "scud info system".green().italic(),
            bright_yellow_backtick(),
            " is invoked".yellow().italic(),
            black_colon(),
            "  *- ".bright_red().italic(),
            bright_yellow_backtick(),
            "neofetch".cyan().italic(),
            bright_yellow_backtick(),
            " (https://github.com/dylanaraps/neofetch) "
                .black()
                .italic(),
            bright_yellow_dots(),
            " to get system information".yellow().italic(),
            black_period(),
            "  *- ".bright_red().italic(),
            bright_yellow_backtick(),
            "cpufetch".cyan().italic(),
            bright_yellow_backtick(),
            "(https://github.com/Dr-Noob/cpufetch) ".black().italic(),
            bright_yellow_dots(),
            " to get CPU architecture information".yellow().italic(),
            black_period(),
        );
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
