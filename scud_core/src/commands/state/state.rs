use std::time::SystemTime;

use crate::{
    cli::{
        State,
        VCS,
    },
    commands::state::executors::{
        execute_state_breezy,
        execute_state_git,
        execute_state_info,
        execute_state_mercurial,
    },
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    helpers::detect_vcs,
    logging::{
        general::log_execution_time,
        helpers::bright_yellow_backtick,
    },
};

use colored::Colorize;

pub fn state_command(state_options: State, start_time: SystemTime) {
    if state_options.info {
        execute_state_info();
    } else {
        execute_state();
    }

    log_execution_time(start_time);
}

// TODO refactor to scud_version_control crate

fn execute_state() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_state_git(),
        VCS::Mercurial => execute_state_mercurial(),
        VCS::Breezy => execute_state_breezy(),
    }

    log_diagnostic(DiagnosticKind::Tip {
        body: &format!(
            "{} {}{}{} {} {}{}{} {}",
            "Use".yellow(),
            bright_yellow_backtick(),
            "scud stage".green().italic(),
            bright_yellow_backtick(),
            "to stage all unstaged changes and untracked files for commit or use"
                .yellow(),
            bright_yellow_backtick(),
            "scud commit".green().italic(),
            bright_yellow_backtick(),
            "to commit any staged changes".yellow()
        ),
    });
}
