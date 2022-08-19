use clap::Args;

/// Initializes a local repository with a given VCS provider (currently
/// supported: git, mercurial, breezy).
///
/// [alias: i]
///
/// Properties:
///
/// * `dry_run`: The name of the new project. [default: false] | [alias: -d].
/// * `name`: The name of the new project. (optional) | [alias: n].
/// * `info`: When true, will output the commands that scud runs under the hood
/// for each of the supported version control systems. (optional) | [default:
/// false]
#[derive(Debug, Args)]

pub struct Init {
    /// When true, will not initialize a new repository with the specified.
    /// VCS provider, but will show expected output.
    /// (optional).
    /// [default: false]
    /// [alias: -d]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// The name of the new project.
    /// (optional).
    /// [alias: n]
    #[clap(short, long, value_parser, required = false)]
    pub name: Option<String>,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
    // /// The desired VCS provider (currently supported: Git, SVN, CVS, Mercurial,
    // Bazaar) /// The default is Git.
    // #[clap(value_parser, default_value_t = String::from("git"))]
    // pub vcs: String,
}

/// Creates a new local repository in the current directory
/// with a specified VCS, if one does not already exist (local repo).
/// Additionally, initializes a corresponding remote repository
/// with a specified source control provider.
///
/// [alias: n]
///
/// Properties:
///
/// * `name`: String that determines the name of the new project.
#[derive(Debug, Args)]
pub struct New {
    /// The name of the new project
    #[clap(value_parser)]
    pub name: String,
    // /// The path to the new project
    // #[clap(value_parser)]
    // pub path: Option<PathBuf>,
}

/// Show changes between the working tree and the index or a tree
///
/// [alias: d]
///
/// Properties:
///
/// * `info`: When true, will output the commands that scud runs under the hood
/// for each of the supported version control systems.
#[derive(Debug, Args)]
pub struct Diff {
    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}
