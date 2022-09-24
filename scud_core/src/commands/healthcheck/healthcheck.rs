use colored::Colorize;

use crate::commands::healthcheck::{
    helpers::{
        healthcheck_sc_github, healthcheck_sc_gitlab, healthcheck_vcs_brz,
        healthcheck_vcs_git, healthcheck_vcs_hg,
    },
    logging::log_healthcheck_note,
};

pub fn healthcheck_command() {
    log_healthcheck_note();

    // Check for installations of VCSs
    execute_healthcheck_vcs();

    // Check for installations of CLIs for source control providers
    execute_healthcheck_source_control();
}

pub fn execute_healthcheck_vcs() {
    println!(
        "{}{}\n",
        "Version Control Systems: ".bright_yellow(),
        "(Git, Mercurial, and Breezy are currently supported)"
            .blue()
            .italic()
    );
    healthcheck_vcs_git();
    healthcheck_vcs_hg();
    healthcheck_vcs_brz();
}

pub fn execute_healthcheck_source_control() {
    println!(
        "\n{}{}",
        "Source Control Providers:".bright_yellow(),
        " (GitHub, GitLab are currently supported)".blue().italic()
    );
    healthcheck_sc_github();
    healthcheck_sc_gitlab();
}

// TODO print after all are successful
// println!("{}", "All supported Version Control Systems are healthy".green());
