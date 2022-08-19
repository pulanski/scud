use clap::{Args, Subcommand};

/// Useful information and diagnostics about your system and codebase
/// (e.g. versions of various tools, system architecture and configuration,
/// counting lines of code, etc.). [alias: in]
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Info {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub command: Option<InfoCommands>,
}

/// The subcommands within scud's info command surface (i.e. codebase, system,
/// cpu, all).
#[derive(Debug, Subcommand)]
pub enum InfoCommands {
    /// Details information about the contents of the codebase within the
    /// current directory [alias: cb]
    #[clap(alias = "cb")]
    Codebase(Codebase),

    /// Details information about the system on which scud is running [alias:
    /// sys] [alias: sys]
    #[clap(alias = "sys")]
    System(System),
}

#[derive(Debug, Args)]
pub struct Codebase {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    #[clap(value_parser)]
    pub info: bool,
}

#[derive(Debug, Args)]
pub struct System {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    #[clap(value_parser)]
    pub info: bool,
}
