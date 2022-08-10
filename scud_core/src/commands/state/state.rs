use std::time::SystemTime;

use crate::{
    cli::{State, VCS},
    commands::state::executors::{
        execute_state_breezy, execute_state_git, execute_state_mercurial,
    },
    diagnostics::{log_diagnostic, DiagnosticKind},
    helpers::detect_vcs,
    logging::{general::log_execution_time, helpers::bright_yellow_backtick},
};

use colored::Colorize;

pub fn state_command(state_options: State, start_time: SystemTime) {
    if state_options.info {
        log_diagnostic(DiagnosticKind::VCSInfo {
            command_name: "state",
            git_command: &format!(
                "{} {}",
                "git status",
                "(along with other commands for more rich output)".bright_yellow()
            ),
            mercurial_command: &format!(
                "{} {}",
                "hg status",
                "(along with other commands for more rich output)".bright_yellow()
            ),
            breezy_command: &format!(
                "{} {}",
                "bzr status",
                "(along with other commands for more rich output)".bright_yellow()
            ),
        });
    } else {
        execute_state();
    }

    log_execution_time(start_time);
}

// TODO refactor to scud_version_control crate

fn execute_state() {
    log_diagnostic(
        DiagnosticKind::CommandInfo {
            command: "state",
            description: "This command is intended to display repository metadata including branching information as well as file changes in a concise, human-readable format."
        }
    );

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
            "to stage all unstaged changes and untracked files for commit or use".yellow(),
            bright_yellow_backtick(),
            "scud commit".green().italic(),
            bright_yellow_backtick(),
            "to commit any staged changes".yellow()
        ),
    });
}
