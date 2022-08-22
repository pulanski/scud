use clap::{
    Args,
    Subcommand,
};

///////////////////////////////////
// Branch subcommands            //
//                               //
// 1. List all branches          //
//      (local and remote)       //
// 2. Rename a branch            //
// 3. Delete a branch            //
//      (local or remote)        //
///////////////////////////////////

/// Primary command for working with branches
/// within the context of scud in the current repository.
/// Handles listing, renaming, and deleting branches.
/// [alias: br]
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Branch {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub command: Option<BranchCommands>,
}

/// The subcommands within scud's branch command surface (i.e. list, delete,
/// rename).
#[derive(Debug, Subcommand)]
pub enum BranchCommands {
    List(BranchList),
    Rename(BranchRename),
    Delete(BranchDelete),
}

/// Lists all branches (local and remote) in the current repository.
/// [alias: ls]
#[derive(Debug, Args)]
pub struct BranchList {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

/// Lists all feature branches in the current local repository.
/// [alias: ]
#[derive(Debug, Args)]
pub struct BranchRename {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

/// Delete a branch in the current repository.
/// CLI will ask if you want to delete remote or local and prompt for
/// confirmation before deleting the branch. [alias: d]
#[derive(Debug, Args)]
pub struct BranchDelete {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub info: bool,
}

///////////////////////////////////
// Feature subcommands           //
//                               //
// 1. List all feature branches  //
// 2. Start a new feature branch //
// 3. Finish a feature branch    //
///////////////////////////////////

/// Provides feature branch functionality following the git-flow branching
/// model. Handles listing, starting, and finishing feature branches.
/// [alias: f]
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Feature {
    /// The subcommand to run.
    #[clap(subcommand)]
    pub command: Option<FeatureCommands>,
}

/// The subcommands within scud's feature command surface (i.e. list, start,
/// finish).
#[derive(Debug, Subcommand)]
pub enum FeatureCommands {
    /// Lists all feature branches in the current local repository.
    /// [alias: ls]
    #[clap(alias = "ls")]
    List(FeatureList),

    /// Starts a new feature branch in the current local repository.
    /// [alias: st]
    #[clap(alias = "st")]
    Start(FeatureStart),
    // /// Finishes the current feature branch in the current local repository.
    // /// [alias: fin]
    // #[clap(alias = "fin")]
    // Finish(Finish),
}

/// Lists all feature branches in the current local repository.
/// [alias: ls]
#[derive(Debug, Args)]
pub struct FeatureList {
    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    #[clap(value_parser)]
    pub info: bool,
}

/// Starts a new feature branch in the current local repository.
#[derive(Debug, Args)]
pub struct FeatureStart {
    /// When true, will not start a feature branch but will show expected
    /// output. (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub dry_run: bool,

    /// When true, will output the commands that scud runs under the hood
    ///
    /// (optional).
    /// [default: false]
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    #[clap(value_parser)]
    pub info: bool,
}
