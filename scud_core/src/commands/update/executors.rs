use std::{
    process::Command,
    time::Duration,
};

use colored::Colorize;
use update_informer::{
    registry,
    Check,
};

pub fn execute_update() {
    // const EVERY_MIN: Duration = Duration::from_secs(60);
    const EVERY_HOUR: Duration = Duration::from_secs(60 * 60);

    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let informer = update_informer::new(registry::Crates, pkg_name, current_version)
        .interval(EVERY_HOUR);

    if let Ok(Some(_new_version)) = informer.check_version() {
        match Command::new("cargo")
            .arg("install")
            .arg("scud")
            .arg("--force")
            .status()
        {
            Ok(status) => {
                if !status.success() {
                    panic!("Failed to stage files");
                } else {
                    // log_repo_status(false);
                }
            }
            Err(error) => {
                panic!("Failed to stage files: {}", error);
            }
        }
        // TODO log version change
    } else {
        println!("{}", "No new release available".green());
        println!(
            "{}{}{}{}{}{}{}",
            " INFO ".black().on_yellow(),
            " A new release of ".yellow().bold(),
            "scud".yellow().italic(),
            " is available: ".yellow().bold(),
            "0.3.0".red().bold().italic(),
            " -> ".black(),
            "0.6.0".to_string().green().bold().italic()
        )
    }
}

pub fn execute_update_dry_run() {
    println!("{}", "Dry run".green());
}

pub fn execute_update_info() {
    println!("{}", "Info".green());
}
