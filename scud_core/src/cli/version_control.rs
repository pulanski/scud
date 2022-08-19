use clap::Args;

/////////////////////////////////////////
// Arguments for the `new` subcommand. //
/////////////////////////////////////////

/// Creates a new local repository in the current directory
/// with a specified VCS, if one does not already exist (local repo).
/// Additionally, initializes a corresponding remote repository
/// with a specified source control provider.
/// [alias: n]
#[derive(Debug, Args)]
pub struct New {
    /// The name of the new project
    #[clap(value_parser)]
    pub name: String,
    // /// The path to the new project
    // #[clap(value_parser)]
    // pub path: Option<PathBuf>,
}
