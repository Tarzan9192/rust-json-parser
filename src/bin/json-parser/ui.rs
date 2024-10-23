use json_parser::io::clear_screen;
use std::io;

/// Prompt the user for a filepath to the json file
/// Returns the parsed file path
pub fn read_input() -> String {
    clear_screen();

    // String to hold the user input
    let mut file_path = String::new();
    println!("Enter file path: ");

    // Keep prompting until we recieve no Err
    while io::stdin().read_line(&mut file_path).is_err() {
        clear_screen();

        // clear the String buffer
        file_path = String::new();

        println!("ERROR READING FILE PATH! Enter a valid file path: ");
    }

    // Trim the newline character and return
    file_path.trim().to_owned()
}
/// Prompt user to hit enter.
/// Throw away returned line.
pub fn press_enter_to_continue() {
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}
