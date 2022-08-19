use std::{
    process::exit,
    time::Duration,
};

use colored::Colorize;
use update_informer::{
    registry,
    Check,
};

use crate::{
    cli::cli::VCS,
    diagnostics::{
        log_diagnostic,
        DiagnosticKind,
    },
    logging::helpers::bright_yellow_backtick,
};

// Where there are options for pure shell scripts, those are chosen
// and associated command is shown to the user (maybe ?)
// general approach will be
// TODO list for supported package managers under the hood for various setup
// commands TODO brew for mac, choco for windows
// make assumption that most people in software dev space are familiar
// with package manager and use the underlying package managers provided
// for usage
// show relevant documentation for installing each package manager
// provide general advice that package managers will save you time
// can be thought of in a manner analogous to app stores for phones
// centralized authority which distributes software (NOTE: can still use more traditional installation techniques as well, e.g. installing Docker from https://www.docker.com/).

pub fn check_version() {
    // const EVERY_MIN: Duration = Duration::from_secs(60);
    const EVERY_DAY: Duration = Duration::from_secs(60 * 60 * 24);

    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let informer = update_informer::new(registry::Crates, pkg_name, current_version)
        .interval(EVERY_DAY);

    if let Ok(Some(new_version)) = informer.check_version() {
        println!(
            "{}{}{}{}{}{}{}\n",
            " INFO ".on_yellow(),
            " A new release of ".bright_yellow(),
            "scud".yellow().italic(),
            " is available: ".bright_yellow(),
            current_version.bright_red().italic(),
            " -> ".black(),
            new_version.to_string().bright_green().italic()
        );
    }
}

// Used in every version-control oriented subcommand
// to determine which underlying VCS to
// use during the execution process
pub fn detect_vcs() -> VCS {
    let mut git = false;
    let mut hg = false;
    let mut bzr = false;

    let invocation_cwd = std::env::current_dir().unwrap();

    let mut current_directory = std::env::current_dir().unwrap();

    #[allow(unused_assignments)]
    let mut parent_directory = current_directory.parent();

    loop {
        current_directory = std::env::current_dir().unwrap();
        parent_directory = current_directory.parent();

        if git | hg | bzr {
            break;
        }

        match parent_directory {
            Some(parent_directory) => {
                if parent_directory == current_directory {
                    break;
                }
                // println!("current: {}", current_directory.display());
                // println!("parent: {}\n", parent_directory.display());
                std::env::set_current_dir(parent_directory).unwrap();

                let dir_contents = std::fs::read_dir(current_directory).unwrap();

                for entry in dir_contents {
                    let entry = entry.unwrap().file_name();
                    if entry.to_string_lossy().eq(".git") {
                        git = true;
                        break;
                    } else if entry.to_string_lossy().eq(".hg") {
                        hg = true;
                        break;
                    } else if entry.to_string_lossy().eq(".bzr") {
                        bzr = true;
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    // Rest the current directory back to the invocation directory
    std::env::set_current_dir(invocation_cwd).unwrap();

    if git {
        VCS::Git
    } else if hg {
        VCS::Mercurial
    } else if bzr {
        VCS::Breezy
    } else {
        log_diagnostic(DiagnosticKind::Error {
            subject: &format!(
                "{} {}",
                "Could not detect a valid version control system",
                "(Supported VCSs are: Git, Mercurial, and Breezy)"
                    .bright_cyan()
                    .italic()
            ),
            body:    "Scud's declarative, high-level operations on top of the \
                      underlying VCS are intended to be used with a supported \
                      version control system.",
        });

        log_diagnostic(DiagnosticKind::Tip {
            body: &format!(
                "{} {}{}{} {}",
                "To get started using scud's declarative, high-level operations on \
                 top of the underlying VCS, try using"
                    .yellow(),
                bright_yellow_backtick(),
                "scud init".green().italic(),
                bright_yellow_backtick(),
                "to initialize a new project with your desired VCS".yellow()
            ),
        });
        exit(1);
    }
}
