use crate::{
    cli::cli::Update,
    commands::update::executors::{
        execute_update, execute_update_dry_run, execute_update_info,
    },
};

pub fn update_command(update_options: Update) {
    if update_options.dry_run {
        execute_update_dry_run();
    } else if update_options.info {
        execute_update_info();
    } else {
        execute_update();
    }
}
