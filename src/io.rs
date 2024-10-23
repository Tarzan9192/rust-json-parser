use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use std::{
    env,
    io::{self},
};

/// Clear the terminal screen.
pub fn clear_screen() {
    let _ = io::stdout().execute(Clear(ClearType::All));
}

/// Parse arguments captured from stdin.
/// Return arguments as a Vec<String>.
/// Skips the first argument when `skip_first == true`.
pub fn parse_args(skip_first: bool) -> Vec<String> {
    if skip_first {
        // skip the first arg since it is usually the program name
        env::args().skip(1).collect()
    } else {
        env::args().collect()
    }
}
