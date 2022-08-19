// Useful information in the context of the codebase and overall system
pub mod info;

// Handles the processing of commands parsed from the command line.
// pub mod process_commands;

pub mod init;

// Declarative high-level operations on the top of the VCS.
pub mod commit;
pub mod diff;
pub mod feature;
pub mod push;
pub mod stage;
pub mod state;
pub mod unstage;

// Handles ensuring the system is setup for usage with scud.
pub mod healthcheck;
pub mod update;
