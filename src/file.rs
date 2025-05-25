use std::fs;

use error::insertion_error::InsertError;

pub fn read_file(file_path: &str) -> Result<String, InsertError> {
    fs::read_to_string(file_path).map_err(|e| InsertError::io_with_path(e, file_path))
}

pub fn _write_file() {}
