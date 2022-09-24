use colored::Colorize;
use dialoguer::{
    theme::ColorfulTheme,
    // Input,
    // Confirm,
    FuzzySelect,
};

use crate::version_control::Init;

pub fn init_command(init_options: Init) {
    let vcs_options = &["Git", "Mercurial", "Breezy"];

    // println!("{}", "Initializing new repository...".green());
    match init_options.name {
        Some(name) => {
            println!(
                "{}",
                format!("Initializing new repository: {}", name).green()
            );
        }
        None => {
            println!("{}", "Initializing new repository...".green());
        }
    }

    let selected_vcs = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Select an underlying VCS (version control system) for the repository",
        )
        .default(0)
        .items(&vcs_options[..])
        .interact()
        .unwrap();

    match selected_vcs {
        0 => {
            println!("Git");
        }
        1 => {
            println!("Mercurial");
        }
        2 => {
            println!("Breezy");
        }
        _ => {
            println!("Unknown");
        }
    }
}
