use std::time::SystemTime;

use colored::Colorize;

pub fn log_execution_time(start_time: SystemTime) {
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();

    if duration.as_secs() > 0 {
        println!(
            "{}{}{}",
            "💥 Done in ".italic(),
            end_time
                .duration_since(start_time)
                .unwrap()
                .as_secs_f32()
                .to_string()
                .italic(),
            "s.".italic(),
        );
    } else {
        println!(
            "{}{}{}",
            "💥 Done in ".italic(),
            end_time
                .duration_since(start_time)
                .unwrap()
                .as_millis()
                .to_string()
                .italic(),
            "ms.".italic(),
        );
    }
}

// pub fn log_dry_run_note() {
//     log_diagnostic(DiagnosticKind::DryRun { command:  });
// }
