use crate::diagnostics::{
    log_diagnostic,
    DiagnosticKind,
};

pub fn execute_branch_list_breezy() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "branch list (breezy)",
    });
}

pub fn execute_branch_list_git() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "branch list (git)",
    });
}

pub fn execute_branch_list_info() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "branch list (info)",
    });
}

pub fn execute_branch_list_mercurial() {
    log_diagnostic(DiagnosticKind::WorkInProgress {
        feature: "branch list (mercurial)",
    });
}
