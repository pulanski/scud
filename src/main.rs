use std::time::Instant;

use clap::Parser;

use scud_core::{
    cli::cli::Cli, general::log_execution_time, helpers::check_version,
    process_commands::process_args,
};

fn main() {
    let start_time = Instant::now();

    // Parse and process command line arguments
    let args = Cli::parse();
    process_args(args);

    // log the execution time
    log_execution_time(start_time);

    // Check for scud updates and notify user when available
    check_version();
}
