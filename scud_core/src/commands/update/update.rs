use std::time::SystemTime;

use crate::{
    cli::Update,
    commands::update::executors::{
        execute_update,
        execute_update_dry_run,
        execute_update_info,
    },
    general::log_execution_time,
};

pub fn update_command(update_options: Update, start_time: SystemTime) {
    if update_options.dry_run {
        execute_update_dry_run();
    } else if update_options.info {
        execute_update_info();
    } else {
        execute_update();
    }

    log_execution_time(start_time);
}
