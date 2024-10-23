use std::{fs::File, io::BufReader};

/// Attempt to open the file at `file_path` and return a BufReader<File>.
pub fn open_file(file_path: &str) -> Option<BufReader<File>> {
    // Open the file and read contents
    // Keep trying until we successfully open a file
    if let Ok(file) = File::open(file_path) {
        // Create a BufReader from the File
        Some(BufReader::new(file))
    } else {
        None
    }
}
