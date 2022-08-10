use std::time::SystemTime;

use colored::Colorize;

use crate::{
    cli::{Stage, VCS},
    commands::stage::executors::{
        execute_stage_breezy, execute_stage_git, execute_stage_mercurial,
    },
    diagnostics::{log_diagnostic, DiagnosticKind},
    helpers::detect_vcs,
    logging::{
        general::{log_dry_run_note, log_execution_time},
        helpers::bright_yellow_backtick,
    },
};

pub fn stage_command(stage_options: Stage, start_time: SystemTime) {
    if stage_options.dry_run {
        execute_stage_dry_run();
        log_dry_run_note();
    } else if stage_options.info {
        execute_stage_info();
    } else {
        execute_stage();
    }

    log_execution_time(start_time);
}

pub fn execute_stage() {
    log_diagnostic(
        DiagnosticKind::CommandInfo {
            command: "stage",
            description: "This command is intended to stage all modified files in the current local repository ensuring they are ready to be committed. It is smart enough to detect the underlying version control system in which it is being run and execute the associated commands. Additionally, it can be run within any deeply nested subdirectory of the local repository root and will stage all files."
        }
    );

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

fn execute_stage_dry_run() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "scud stage --dry-run",
    });
}

fn execute_stage_info() {
    log_diagnostic(
        DiagnosticKind::VCSInfo {
            command_name: "stage",
            git_command: "git add -A",
            mercurial_command: "Algorithm pseudocode:\n\t1. (Navigate to root of repository)\n\t2. hg add .\n\t3. (Navigate back to invocation directory)",
            breezy_command: "Algorithm pseudocode:\n\t1. (Navigate to root of repository)\n\t2. bzr add .\n\t3. (Navigate back to invocation directory)",
        },
    );
}
