use std::time::SystemTime;

use colored::Colorize;
use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme,
    // Input,
    // Confirm,
};

use crate::{
    cli::Init,
    logging::general::log_execution_time,
};

pub fn init_command(init_options: Init, start_time: SystemTime) {

    let vcs_options = &[
        "Git",
        "Mercurial",
        "Breezy",
    ];

    // println!("{}", "Initializing new repository...".green());
    match init_options.name {
      Some(name) => {
        println!("{}", format!("Initializing new repository: {}", name).green());
      }
      None => {
        println!("{}", "Initializing new repository...".green());
      }
    }

    let selected_vcs = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an underlying VCS (version control system) for the repository")
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

     log_execution_time(start_time);
}
