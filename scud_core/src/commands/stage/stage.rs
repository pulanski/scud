use colored::Colorize;

use crate::{
    cli::cli::{Stage, VCS},
    commands::stage::executors::{
        execute_stage_breezy, execute_stage_dry_run, execute_stage_git,
        execute_stage_info, execute_stage_mercurial,
    },
    diagnostics::{log_diagnostic, DiagnosticKind},
    helpers::detect_vcs,
    logging::helpers::bright_yellow_backtick,
};

pub fn stage_command(stage_options: Stage) {
    if stage_options.dry_run {
        execute_stage_dry_run();
    } else if stage_options.info {
        execute_stage_info();
    } else {
        execute_stage();
    }
}

pub fn execute_stage() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_stage_git(),
        VCS::Mercurial => execute_stage_mercurial(),
        VCS::Breezy => execute_stage_breezy(),
    }

    log_diagnostic(DiagnosticKind::Tip {
        body: &format!(
            "{} {}{}{} {}",
            "Use".yellow(),
            bright_yellow_backtick(),
            "scud commit".green().italic(),
            bright_yellow_backtick(),
            "to commit any staged changes".yellow()
        ),
    });
}
