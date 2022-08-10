use std::{
    time::SystemTime,
};

use clap::Parser;

use scud_core::{
    cli::Cli,
    process_commands::process_args,
    helpers::check_version,
};

// TODO View open issues in current repository

fn main() {
    let start_time = SystemTime::now();

    // Parse and process command line arguments
    let args = Cli::parse();
    process_args(args, start_time);

    // Check for scud updates and notify user when available
    check_version();
}
