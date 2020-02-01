use std::fs::File;
use std::path::Path;

/// Opens up the given file in read-only mode. Panics if an error occurs.
pub fn open_file(filename: String) -> File {
    // Open up the file (read-only)
    let filepath = Path::new(&filename);
    let file = match File::open(&filepath) {
        Err(e) => panic!("ERROR - couldn't open {}. ({})", filepath.display(),
            e.to_string()),
        Ok(file) => file,
    };
    return file;
}
