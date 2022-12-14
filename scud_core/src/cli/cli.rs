use crate::{
    branching::{Branch, Feature},
    cli::version_control::{Diff, Init, New},
};

use clap::{Args, Parser, Subcommand};

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
    // Bazaar, not actively maintained, Breezy is recommended alternative and is
    // supported instead.
}

/// Command Line Argument Parser for scud
#[derive(Debug, Parser)]
#[clap(name = "scud")]
#[clap(
    about = "A tool for streamlining the version control processes of your \
             development workflow.",
    long_about = "A tool for streamlining the many version control processes of \
                  your development workflow written with an emphasis on \
                  ease-of-use and expressive usage diagnostics."
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
    // Useful information and diagnostics about your system and codebase
    // [alias: in]
    // #[clap(alias = "in")]
    // Info(Info),
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
    /// supported: git). [alias: i]
    // TODO Additionally, asks you if you would like to initialize the
    // repository with a basic branch structure following the GitFlow
    // branching model.
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
    // TODO
    // #[clap(alias = "su")]
    // Setup(Setup),

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
    /// untracked, staged, etc.).
    /// [alias: st]
    // This command is useful for checking the status of the local repository in
    // terms of seeing which files are untracked, staged, etc.
    #[clap(alias = "st")]
    State(State),

    /// Primary subcommand for working with branches.
    /// Handles general CRUD operations with branches.
    /// [alias: br]
    #[clap(alias = "br")]
    Branch(Branch),

    /// Provides feature branch functionality following the git-flow branching
    /// model. Handles listing, starting, and finishing feature branches.
    /// [alias: f]
    #[clap(alias = "f")]
    Feature(Feature),

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

    // Powerful one-liner which can be thought of as
    // sequentially running the following commands:
    //
    // 1. scud stage
    // 2. scud commit
    // 3. scud push
    #[clap(alias = "stream")]
    Upstream(Upstream),

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
/// ??????????????????
#[derive(Debug, Args)]
pub struct State {
    /// When true, will output the commands that scud runs under the hood
    /// for each of the supported version control systems.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

// TODO add documentation for this command
#[derive(Debug, Args)]
pub struct Upstream {
    /// When true, will output the commands that scud runs under the hood.
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

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
