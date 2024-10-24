mod ui;

use file_manager::*;
use json_parser::io::*;
use json_parser::json::*;
use ui::*;

pub fn main() {
    run();
}

fn run() {
    // First check command line args
    let args = parse_args(true);
    if let Some(file_path) = args.first() {
        if let Some(file) = open_file(file_path) {
            if let Some(parsed) = parse(file) {
                println!("Parsed: {:?}", parsed);
            }
        } else {
            println!("File not found!")
        }
    } else {
        // Else prompt the user for path until a valid one is supplied
        loop {
            let file_path = read_input();
            if let Some(file) = open_file(file_path.as_str()) {
                if let Some(parsed) = parse(file) {
                    println!("Parsed: {:?}", parsed);
                    break;
                }
            } else {
                println!("File not found!");
                press_enter_to_continue();
            }
        }
    }
}
