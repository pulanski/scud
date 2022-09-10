use std::time::SystemTime;

use colored::Colorize;

use crate::{
    cli::cli::{Unstage, VCS},
    commands::unstage::executors::execute_unstage_git,
    detect_vcs,
    diagnostics::{log_diagnostic, DiagnosticKind},
    logging::{general::log_execution_time, helpers::bright_yellow_backtick},
};

pub fn unstage_command(unstage_options: Unstage, start_time: SystemTime) {
    if unstage_options.dry_run {
        execute_unstage_dry_run();
    } else if unstage_options.info {
        execute_unstage_info();
    } else {
        execute_unstage();
    }

    log_execution_time(start_time);
}

fn execute_unstage() {
    log_diagnostic(DiagnosticKind::ScudCommandInfo {
        command: "unstage",
        description: "This command is intended to unstage all modified files in \
                      the current local repository ensuring they are ready to be \
                      committed. It is smart enough to detect the underlying \
                      version control system in which it is being run and execute \
                      the associated commands. Additionally, it can be run within \
                      any deeply nested subdirectory of the local repository root \
                      and will unstage all modified files and directories within \
                      the current repository.",
    });

    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_unstage_git(),
        VCS::Mercurial => println!("{}", "Mercurial repository detected".green()),
        VCS::Breezy => println!("{}", "Breezy repository detected".green()),
    }

    log_diagnostic(DiagnosticKind::Tip {
        body: &format!(
            "{} {}{}{} {}",
            "When you're ready, use".yellow(),
            bright_yellow_backtick(),
            "scud stage".green().italic(),
            bright_yellow_backtick(),
            "to stage all unstaged changes and untracked files for commit".yellow(),
        ),
    });
}

pub fn execute_unstage_dry_run() {
    log_diagnostic(DiagnosticKind::DryRun { command: "unstage" });
}

fn execute_unstage_info() {}
