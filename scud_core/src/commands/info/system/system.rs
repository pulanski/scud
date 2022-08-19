use std::time::SystemTime;

use crate::{
    general::log_execution_time,
    System,
};

pub fn system_command(system_options: System, start_time: SystemTime) {
    if system_options.info {
        println!("System: info");
    } else {
        execute_system();
    }

    log_execution_time(start_time);
}

pub fn execute_system() {
    println!("TODO");
}
