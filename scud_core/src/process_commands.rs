use std::time::SystemTime;

use crate::{
    cli::*,
    commands::{
        commit::commit::commit_command, healthcheck::healthcheck::healthcheck_command,
        info::codebase::codebase::codebase_command, init::init::init_command,
        push::push::push_command, stage::stage::stage_command, state::state::state_command,
        unstage::unstage::unstage_command, update::update::update_command,
    },
    info,
};

pub fn process_args(args: Cli, start_time: SystemTime) {
    let args = args.command;

    match args {
        Commands::Info(info_commands) => {
            let info_command = info_commands.command.unwrap();

            match info_command {
                InfoCommands::Codebase(codebase_options) => {
                    codebase_command(codebase_options, start_time)
                }
            }
        }

        //////////////////////////////////////////////////////////////
        // Commands for starting repositories (local, remote, both) //
        //////////////////////////////////////////////////////////////
        Commands::New(new_repo_options) => {
            println!("New: {}", new_repo_options.name);
        }
        Commands::Init(init_options) => {
            init_command(init_options, start_time);
        }

        //////////////////////////////////////////////////////////////
        // Declarative high-level operations on the top of the VCS. //
        //////////////////////////////////////////////////////////////
        Commands::State(state_options) => {
            state_command(state_options, start_time);
        }

        Commands::Stage(stage_options) => {
            stage_command(stage_options, start_time);
        }

        Commands::Unstage(unstage_options) => {
            unstage_command(unstage_options, start_time);
        }

        Commands::Commit(commit_options) => {
            commit_command(commit_options);
        }

        Commands::Push(push_options) => {
            push_command(push_options, start_time);
        }

        Commands::Pull(pull_options) => {
            println!("Pull: {}", pull_options.info);
            // pull_command(pull_options, start_time);
        }

        ////////////////////////////
        // Various setup commands //
        ////////////////////////////
        Commands::Setup(setup_options) => {
            println!("Setup {}", setup_options.info);
        }

        //////////////////////////////////////////////////
        // Commands to ensure system is setup for usage //
        // w/ the latest version of scud                //
        //////////////////////////////////////////////////
        Commands::Healthcheck => {
            healthcheck_command(start_time);
        }

        Commands::Update(update_options) => {
            update_command(update_options, start_time);
        }

        /////////////////////////////////////////////
        // Handler for processing unknown commands //
        /////////////////////////////////////////////
        _ => {
            println!("Unknown command: {:?}", args);
        }
    }
}
