use std::time::Instant;

use colored::Colorize;

pub fn log_execution_time(start_time: Instant) {
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    if duration.as_secs() > 0 {
        println!(
            "{}{}{}",
            "💥 Done in ".italic(),
            end_time
                .duration_since(start_time)
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
                .as_millis()
                .to_string()
                .italic(),
            "ms.".italic(),
        );
    }
}
