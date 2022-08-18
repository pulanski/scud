use std::path::PathBuf;

use clap::{
    Args,
    Parser,
    Subcommand,
};

// TODO add scud stash
// add scud pop
// generate changelog on release
// scud release
// https://git-scm.com/book/en/v2/Git-Basics-Tagging
// on commit, generate lightweight tag
// on release, generate annotated tag
// cargo semver bump patch
// check if semver installed
// if not installed ask to install
// if they want to install, install it
// else don't install it and log that package_version wasn't updated
// need to setup semver

pub enum VCS {
    Git,
    Mercurial,
    Breezy,
    // SVN, // TODO add support for SVN (maybe), requires different strategy
    // CVS, // TODO add support for CVS (maybe), requires different strategy for
    // detection Bazaar, not actively maintained, Breezy is recommended
    // alternative.
}

/// Command Line Argument Parser for scud
#[derive(Debug, Parser)]
#[clap(name = "scud")]
#[clap(
    about = "A toolkit for streamlining the many version and source control \
             processes of your development workflow. Agnostic to your codebase's \
             internals and development environment, it just works.",
    long_about = "Scud was created to fill the gap between the many version \
                  control processes of your development workflow and build upon \
                  the features provided by similar tools in the space (commitizen, \
                  cz cli, etc.), without compromising on performance. All commands \
                  support aliases thanks to clap to further enhance overall DX."
)]
#[clap(version)]
pub struct Cli {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub command: Commands,
}

/// The subcommands within scud's CLI surface.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Useful information and diagnostics about your system and codebase
    /// [alias: in]
    #[clap(alias = "in")]
    Info(Info),

    /// Creates a new local repository in the current directory
    /// with a specified VCS, if one does not already exist (local repo).
    /// Additionally, initializes a corresponding remote repository
    /// with a specified source control provider.
    /// [alias: n]
    // This command is useful for streamlining the entire process of
    // creating a local repository for your given project/app/library
    // and getting a remote repository up and running in a matter of seconds.
    // Supports a variety of different combinations of VCS
    // and source control providers.
    #[clap(alias = "n")]
    New(New),

    /// Initializes a local repository with a given VCS provider (currently
    /// supported: git, mercurial, breezy). [alias: i]
    // (currently supported: Git, SVN, CVS, Mercurial, Bazaar).
    // This command is useful for initializing a repository that
    // is not yet tracked by a particular Version Control System.
    #[clap(alias = "i")]
    Init(Init),

    /// Streamlines the setup process for various version control systems
    /// and source control providers.
    /// [alias: su]
    // This command is useful for quickly setting up various
    // dependencies onto your local system
    // (i.e. git, bazaar, gh, glab, etc.),
    // so you can focus your time on more important things.
    #[clap(alias = "su")]
    Setup(Setup),

    /// Stages all modified files in the current local repository
    /// ensuring they are ready to be committed. This command can be run from
    /// any deeply nested subdirectory of the current local repository
    /// and is smart enough to stage all modified files in the current
    /// local repository.
    /// [alias: s]
    // This command is useful for staging all modified files in your local repository
    #[clap(alias = "s")]
    Stage(Stage),

    /// Unstages all modified files and directories within
    /// the current local repository.
    /// [alias: u]
    // This command is useful for reverting changes made to files tracked by your
    // version control system.
    #[clap(alias = "u")]
    Unstage(Unstage),

    /// Checks the status of the local repository (e.g. seeing which files are
    /// untracked, staged, etc. as well as branching information).
    /// [alias: st]
    // This command is useful for checking the status of the local repository in
    // terms of seeing which files are untracked, staged, etc.
    #[clap(alias = "st")]
    State(State),

    /// Show changes between the working tree and the index or a tree
    /// [alias: d]
    // This command is useful for seeing the changes between the working tree and the
    // index or a tree.
    #[clap(alias = "d")]
    Diff(Diff),

    /// Commits all staged files in the current local repository.
    /// [alias: c]
    // This command is useful when you have reached a codebase state you
    // want to remember
    #[clap(alias = "c")]
    Commit(Commit),

    /// Stages all modified files in the current local repository
    /// and then commits them.
    /// [alias: ca]
    // This command is useful for further streamlining the stage
    // and commit process.
    #[clap(alias = "ca")]
    CommitAll(CommitAll),

    /// Pushes all commits to the remote repository.
    /// [alias: ps]
    // This command is useful for pushing your local commits to the remote
    // repository.
    #[clap(alias = "ps")]
    Push(Push),

    /// Pulls all commits from the remote repository.
    /// [alias: pl]
    // This command is useful for pulling your remote commits to the local
    // repository.
    #[clap(alias = "pl")]
    Pull(Pull),

    /// Handles the process of updating scud to the latest version.
    /// [alias: up]
    // This command is useful for updating scud to the latest version.
    #[clap(alias = "up")]
    Update(Update),

    /// Checks to see required dependencies (i.e. version control systems,
    /// CLIs for source control providers) are installed.
    /// [alias: hc]
    // This command is useful for checking to see if your system is setup
    // to work with Scud.
    #[clap(alias = "hc")]
    Healthcheck,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Info {
    #[clap(subcommand)]
    pub command: Option<InfoCommands>,
}

#[derive(Debug, Subcommand)]
pub enum InfoCommands {
    /// Details information about the contents of the codebase within the
    /// current directory [alias: cb]
    #[clap(alias = "cb")]
    Codebase(Codebase),
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

    /// The path to the new project
    #[clap(value_parser)]
    pub path: Option<PathBuf>,
}

//////////////////////////////////////////
// Arguments for the `init` subcommand. //
//////////////////////////////////////////

/// Initializes a local repository with a given VCS provider (currently
/// supported: git, mercurial, breezy). [alias: i]
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

///////////////////////////////////////////
// Arguments for the `setup` subcommand. //
///////////////////////////////////////////

/// Streamlines the setup process for various version control systems
/// and source control providers.
/// [alias: su]
#[derive(Debug, Args)]
pub struct Setup {
    /// The desired VCS provider (currently supported: Git, SVN, CVS, Mercurial,
    /// Bazaar) The default is GitHub.
    #[clap(value_parser)]
    pub vcs: Option<String>,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

//////////////////////////////////////////
// Arguments for the `state` subcommand. //
//////////////////////////////////////////

/// Shows changes to files in the current local repository
/// Lists changed, Lists changed, added and deleted files compared to the
/// currently checked-out commit.
///
/// [alias: s]
/// ✔✔️✔️️
#[derive(Debug, Args)]
pub struct State {
    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

#[derive(Debug, Args)]
pub struct Diff {
    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

// / Shellder
// / the layer over your shell
// / healthcheck shows which shells are installed
// /
// / enum PackageManager {
// /     name: String,
// /     href:
// / }
// /
// / shellder setup fish
// / shellder teardown fish

//////////////////////////////////////////
// Arguments for the `stage` subcommand. //
//////////////////////////////////////////

/// Stages all modified files in the current local repository
/// ensuring they are ready to be committed. This command can be run from
/// any deeply nested subdirectory of the current local repository
/// and is smart enough to stage all modified files in the current
/// local repository.
/// [alias: s]
#[derive(Debug, Args)]
pub struct Stage {
    /// When true, will not stage files but will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

/////////////////////////////////////////////
// Arguments for the `unstage` subcommand. //
/////////////////////////////////////////////

/// Unstages all modified files and directories in the current
/// local repository so they are ready to be staged again.
/// [alias: u]
#[derive(Debug, Args)]
pub struct Unstage {
    /// When true, will not unstage files and directories
    /// but will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

////////////////////////////////////////////
// Arguments for the `commit` subcommand. //
////////////////////////////////////////////

/// Commits all staged files in the current local repository.
/// [alias: c]
#[derive(Debug, Args)]
pub struct Commit {
    /// When true, will not commit staged files but will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

///////////////////////////////////////////////
// Arguments for the `commit-all` subcommand. //
///////////////////////////////////////////////

/// Stages all modified files in the current local repository
/// and then commits them.
/// [alias: ca]
#[derive(Debug, Args)]
pub struct CommitAll {
    // /// The commit message to use when committing.
    // /// (optional).
    // #[clap(short, long, value_parser, required = false)]
    // pub message: Option<String>,
    /// When true, will not stage and commit all files but
    /// will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

//////////////////////////////////////
// Arguments for the `push` command //
//////////////////////////////////////

/// Pushes all commits to the remote repository.
/// [alias: ps]
#[derive(Debug, Args)]
pub struct Push {
    /// When true, will not push committed files but
    /// will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

//////////////////////////////////////
// Arguments for the `pull` command //
//////////////////////////////////////

/// Pulls all commits from the remote repository.
/// [alias: pl]
#[derive(Debug, Args)]
pub struct Pull {
    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

//////////////////////////////////////
// Arguments for the `update` command //
//////////////////////////////////////

/// Handles the process of updating scud to the latest version.
/// [alias: up]
#[derive(Debug, Args)]
pub struct Update {
    /// When true, will update scud but
    /// will show expected output.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}
