use crate::{
    cli::cli::VCS,
    commands::diff::executors::{
        execute_diff_breezy, execute_diff_git, execute_diff_info,
        execute_diff_mercurial,
    },
    helpers::detect_vcs,
    version_control::Diff,
};

pub fn diff_command(diff_options: Diff) {
    if diff_options.info {
        execute_diff_info();
    } else {
        execute_diff();
    }
}

pub fn execute_diff() {
    let vcs = detect_vcs();

    match vcs {
        VCS::Git => execute_diff_git(),
        VCS::Mercurial => execute_diff_mercurial(),
        VCS::Breezy => execute_diff_breezy(),
    }

    // TODO look at adding diagnostic here

    // log_diagnostic(DiagnosticKind::Tip {
    //     body: &format!(
    //         "{} {}{}{} {}",
    //         "Use".yellow(),
    //         bright_yellow_backtick(),
    //         "scud commit".green().italic(),
    //         bright_yellow_backtick(),
    //         "to commit any staged changes".yellow()
    //     ),
    // });
}
