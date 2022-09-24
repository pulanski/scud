use crate::{
    cli::cli::{State, VCS},
    commands::state::executors::{
        execute_state_breezy, execute_state_git, execute_state_info,
        execute_state_mercurial,
    },
    diagnostics::{log_diagnostic, DiagnosticKind},
    helpers::detect_vcs,
    logging::helpers::bright_yellow_backtick,
};

use colored::Colorize;

pub fn state_command(state_options: State) {
    if state_options.info {
        execute_state_info();
    } else {
        execute_state();
    }
}

fn execute_state() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_state_git(),
        VCS::Mercurial => execute_state_mercurial(),
        VCS::Breezy => execute_state_breezy(),
    }

    log_diagnostic(DiagnosticKind::Tip {
        body: &format!(
            "{} {}{}{} {}{}{}{}{} {}{}{} {}{}",
            "Use".yellow(),
            bright_yellow_backtick(),
            "scud stage".green().italic(),
            bright_yellow_backtick(),
            "to stage all ".yellow(),
            "unstaged".bright_yellow(),
            " and ".yellow(),
            "untracked changes".bright_yellow(),
            " for commit or use".yellow(),
            bright_yellow_backtick(),
            "scud commit".green().italic(),
            bright_yellow_backtick(),
            "to commit any ".yellow(),
            "staged changes".bright_yellow()
        ),
    });
}
