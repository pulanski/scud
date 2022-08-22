use std::time::SystemTime;

use crate::{
    cli::{
        branching::Branch,
        cli::VCS,
    },
    commands::branch::executors::{
        execute_branch_breezy,
        execute_branch_git,
        execute_branch_info,
        execute_branch_mercurial,
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

pub fn branch_command(branch_options: Branch, start_time: SystemTime) {
    if state_options.info {
        execute_branch_info();
    } else {
        execute_branch();
    }

    log_execution_time(start_time);
}

fn execute_state() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_branch_git(),
        VCS::Mercurial => execute_branch_mercurial(),
        VCS::Breezy => execute_branch_breezy(),
    }

    // TODO - add tip message
}
