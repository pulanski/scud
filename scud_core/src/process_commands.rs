use crate::{
    branch::branch::process_branch_commands,
    cli::cli::{Cli, Commands},
    commands::{
        commit::commit::commit_command, diff::diff::diff_command,
        healthcheck::healthcheck::healthcheck_command, init::init::init_command,
        push::push::push_command, stage::stage::stage_command,
        state::state::state_command, unstage::unstage::unstage_command,
        update::update::update_command,
    },
};

/// Handler for the various CLI commands
pub fn process_args(args: Cli) {
    let args = args.command;

    match args {
        //////////////////////////////////////////////////////////////
        // Commands for starting repositories (local, remote, both) //
        //////////////////////////////////////////////////////////////
        Commands::New(new_repo_options) => {
            println!("New: {}", new_repo_options.name);
        }
        Commands::Init(init_options) => {
            init_command(init_options);
        }

        //////////////////////////////////////////////////////////////
        // Declarative high-level operations on the top of the VCS. //
        // with improved usage information and diagnostics.         //
        // - State                                                  //
        // - Stage                                                  //
        // - Unstage                                                //
        // - Commit                                                 //
        // - Diff                                                   //
        //////////////////////////////////////////////////////////////
        Commands::State(state_options) => {
            state_command(state_options);
        }

        Commands::Stage(stage_options) => {
            stage_command(stage_options);
        }

        Commands::Unstage(unstage_options) => {
            unstage_command(unstage_options);
        }

        Commands::Commit(commit_options) => {
            commit_command(commit_options);
        }

        Commands::Push(push_options) => {
            push_command(push_options);
        }

        Commands::Pull(pull_options) => {
            println!("Pull: {}", pull_options.info);
            // pull_command(pull_options, start_time);
        }

        Commands::Diff(diff_options) => {
            diff_command(diff_options);
        }

        ///////////////////////////////////
        // Branching commands along with //
        // branching strategies          //
        ///////////////////////////////////
        Commands::Branch(branch_commands) => {
            process_branch_commands(branch_commands)
        }

        ////////////////////////////
        // Various setup commands //
        ////////////////////////////
        // Commands::Setup(setup_options) => {
        //     println!("Setup {}", setup_options.info);
        // }

        //////////////////////////////////////////////////
        // Commands to ensure system is setup for usage //
        // w/ the underlying components which make up   //
        // scud                                         //
        //////////////////////////////////////////////////
        Commands::Healthcheck => {
            healthcheck_command();
        }

        //
        Commands::Update(update_options) => {
            update_command(update_options);
        }

        /////////////////////////////////////////////
        // Handler for processing unknown commands //
        /////////////////////////////////////////////
        _ => {
            !unreachable!();
        }
    }
}
