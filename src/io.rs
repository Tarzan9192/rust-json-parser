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

/// Return arguments as a Vec<String>.
/// Skips the first argument.
pub fn parse_args() -> Vec<String> {
    // skip the first arg since it is usually the program name
    env::args().skip(1).collect()
}

