use std::fs::File;
use std::path::Path;
use std::error::Error;

/// Opens up the given file in read-only mode. Panics if an error occurs.
pub fn open_file(filename: String) -> File {
    // Open up the file (read-only)
    let filepath = Path::new(&filename);
    let file = match File::open(&filepath) {
        Err(e) => panic!("ERROR - couldn't open {}. ({})", filepath.display(),
            e.description()),
        Ok(file) => file,
    };
    return file;
}