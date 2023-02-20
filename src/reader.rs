use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum FileExtensionError {
    #[error("File has no extension.")]
    Missing,
    #[error("Unsupported file extension: {0}")]
    Unsupported(String),
    #[error("Supported file extensions list is empty.")]
    SupportedListEmpty,
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(|s| s.to_str())
}

/// Returns Ok(()) if the given file has an extension that is contained in the given extensions list.
//
pub fn check_file_extension(filename: &str, extensions: &[&str]) -> Result<(), FileExtensionError> {
    if extensions.is_empty() {
        return Err(FileExtensionError::SupportedListEmpty);
    }
    let file_ext = match get_file_extension(filename) {
        Some(ext) => ext,
        None => return Err(FileExtensionError::Missing),
    };
    if extensions.contains(&file_ext) {
        Ok(())
    } else {
        Err(FileExtensionError::Unsupported(file_ext.to_owned()))
    }
}

/// Byte container.
//
type Buffer = Vec<u8>;

/// Reads the contents of a binary file into a Buffer
//
pub fn read_file_to_buffer(filename: &str) -> Result<Buffer> {
    let mut file =
        File::open(filename).with_context(|| format!("Failed to open {filename} file."))?;

    let mut buffer = Buffer::new();

    file.read_to_end(&mut buffer)
        .with_context(|| "Failed to read data into buffer.")?;

    Ok(buffer)
}

// alternative solution: read the file into a fixed-sized buffer/array
//
//const BUFFER_SIZE_IN_BYTES: usize = 20000;
//let mut buffer = [0; BUFFER_SIZE_IN_BYTES];
//#[allow(unused_assignments)]
//let mut bytes_read_count: usize = 0;
//{
//    let mut file = File::open("cube.lxo")?;
//    bytes_read_count = file.read(&mut buffer[..])?;
//    assert_eq!(
//        bytes_read_count, EXPECTED_FILE_SIZE_IN_BYTES,
//        "Read all the bytes?"
//    );
//}

// a few tests; not comprehensive.
//
#[cfg(test)]
mod tests {
    use super::*;

    const FILENAME_EMPTY: &str = "";
    const FILENAME_NO_EXTENSION: &str = "model";
    const FILENAME_LXO: &str = "model.lxo";
    const FILENAME_ABC: &str = "model.abc";
    const SUPPORTED_FILE_EXTENSIONS: &[&str] = &["def", "lxo"];

    #[test]
    fn get_file_extension_empty_filename() {
        assert_eq!(get_file_extension(FILENAME_EMPTY), None);
    }

    #[test]
    fn get_file_extension_missing_extension() {
        assert_eq!(get_file_extension(FILENAME_NO_EXTENSION), None);
    }

    #[test]
    fn get_file_extension_has_extension() {
        assert_eq!(get_file_extension(FILENAME_LXO), Some("lxo"));
    }

    #[test]
    fn check_file_extension_empty_extensions_array() {
        assert_eq!(
            check_file_extension(FILENAME_LXO, &[]),
            Err(FileExtensionError::SupportedListEmpty)
        );
    }

    #[test]
    fn check_file_extension_unsupported() {
        assert_eq!(
            check_file_extension(FILENAME_ABC, SUPPORTED_FILE_EXTENSIONS),
            Err(FileExtensionError::Unsupported("abc".to_owned()))
        );
    }

    #[test]
    fn check_file_extension_no_extension() {
        assert_eq!(
            check_file_extension(FILENAME_NO_EXTENSION, SUPPORTED_FILE_EXTENSIONS),
            Err(FileExtensionError::Missing)
        );
    }

    #[test]
    fn check_file_extension_supported_file_type() {
        assert_eq!(
            check_file_extension(FILENAME_LXO, SUPPORTED_FILE_EXTENSIONS),
            Ok(())
        );
    }
}
