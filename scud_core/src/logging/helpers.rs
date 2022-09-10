use colored::{ColoredString, Colorize};

////////////////////////////////////////
// Helpers that effectively provide   //
// syntax highlighting primitives for //
// the logging system.                //
////////////////////////////////////////

pub fn bright_yellow_backtick() -> ColoredString {
    "`".bright_yellow()
}

pub fn yellow_backtick() -> ColoredString {
    "`".yellow()
}

pub fn black_italic_quote() -> ColoredString {
    "\"".black().italic()
}

pub fn black_period() -> ColoredString {
    ".".black()
}

pub fn black_comma() -> ColoredString {
    ",".black()
}

pub fn black_colon() -> ColoredString {
    ":".black()
}

pub fn black_italic_open_paren() -> ColoredString {
    "(".black().italic()
}

pub fn black_italic_close_paren() -> ColoredString {
    ")".black().italic()
}

pub fn black_italic_implies() -> ColoredString {
    "=>".black().italic()
}

pub fn bright_yellow_dots() -> ColoredString {
    "...".bright_yellow().italic()
}
