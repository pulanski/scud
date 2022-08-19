use crate::diagnostics::{log_diagnostic, DiagnosticKind};

pub fn execute_diff_breezy() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "diff (breezy)",
    });
}

pub fn execute_diff_dry_run() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "diff (dry run)",
    });
}

pub fn execute_diff_git() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "diff (git)",
    });
}

pub fn execute_diff_info() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "diff (info)",
    });
}

pub fn execute_diff_mercurial() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "diff (mercurial)",
    });
}
