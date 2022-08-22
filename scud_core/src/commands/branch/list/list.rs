use std::time::SystemTime;

use crate::{
    cli::{
        branching::BranchList,
        cli::VCS,
    },
    commands::branch::list::executors::{
        execute_branch_list_breezy,
        execute_branch_list_git,
        execute_branch_list_info,
        execute_branch_list_mercurial,
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

pub fn branch_list_command(branch_list_options: BranchList, start_time: SystemTime) {
    if branch_list_options.info {
        execute_branch_list_info();
    } else {
        execute_branch_list();
    }

    log_execution_time(start_time);
}

fn execute_branch_list() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_branch_list_git(),
        VCS::Mercurial => execute_branch_list_mercurial(),
        VCS::Breezy => execute_branch_list_breezy(),
    }

    // TODO - add tip message
}
