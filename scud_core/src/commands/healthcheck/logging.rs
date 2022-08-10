use colored::Colorize;

pub fn log_healthcheck_note() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        "{}{}{}\n{}\n",
        "Scud v"
            .yellow(),
        VERSION
            .yellow(),
        " healthcheck"
            .yellow(),
        "This command is intended to ensure that the system is setup for usage with the various features of scud."
            .italic()
    );
}
