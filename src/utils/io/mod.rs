use std::fs::File;
use std::io::Read;

/// Reads and returns the contents of the given file.
pub fn read_file_to_string(file: &mut File) -> String {
    let mut read_buf = String::from("");
    match file.read_to_string(&mut read_buf) {
        Err(e) => panic!("Error reading file. ({})", e.to_string()),
        Ok(_) => 0,
    };
    return read_buf;
}
