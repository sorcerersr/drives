/*
 * This module wraps the std::fs calls within a trait and an
 * implementing struct to be able to mock it with 'mockall'.
 *
 */
use crate::error::DrivesError;
use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::{self, BufRead, Read},
    path::Path,
};

pub fn read_dir(path: &str) -> Result<ReadDir, DrivesError> {
    let readdir = fs::read_dir(path).map_err(|_err| DrivesError::DiraccessError {
        directory: path.to_string(),
    })?;
    Ok(readdir)
}

pub fn read_bool_file(path: &str) -> Result<bool, DrivesError> {
    let content = read_file_to_string(Path::new(path))?;
    Ok("1".eq(&content))
}

pub fn name_from_direntry(entry: &DirEntry) -> Result<String, DrivesError> {
    if let Ok(result) = entry.file_name().into_string() {
        Ok(result)
    } else {
        Err(DrivesError::NameFromDirEntryFailed)
    }
}

pub fn build_path(dir_entry: &DirEntry, path_to_add: &str) -> Result<String, DrivesError> {
    let mut path = if let Some(path) = dir_entry.path().to_str() {
        path.to_string()
    } else {
        return Err(DrivesError::PathAppendFailed);
    };
    path.push_str(path_to_add);
    Ok(path)
}

pub fn read_file_to_string(path: &Path) -> Result<String, DrivesError> {
    let mut file = File::open(path).map_err(|_err| DrivesError::FileAccessError {
        filename: path_to_string(path),
    })?;

    let mut result = String::new();
    file.read_to_string(&mut result);
    result = result.replace('\n', "").trim().to_owned();
    Ok(result)
}

pub fn read_file_to_u64(path: &str) -> Result<u64, DrivesError> {
    let content = read_file_to_string(Path::new(path))?;

    let size_as_u64 = if let Ok(size) = content.parse() {
        size
    } else {
        return Err(DrivesError::ConversionToU64Failed);
    };
    Ok(size_as_u64)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn path_to_string(path: &Path) -> String {
    if let Some(path_str) = path.to_str() {
        path_str.to_string()
    } else {
        "???".to_string()
    }
}

#[cfg(test)]
mod tests {

    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_read_bool_file() {
        // prepare a temporary file to read from
        let mut test_file = NamedTempFile::new().unwrap();
        test_file.write_all("1".as_bytes()).unwrap();

        // call the method under test
        let mut result = read_bool_file(test_file.path().to_str().unwrap());
        assert!(result.unwrap());

        test_file.write_all("0".as_bytes()).unwrap();

        result = read_bool_file(test_file.path().to_str().unwrap());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_read_file_to_string() {
        // prepare a temporary file to read from
        let mut test_file = NamedTempFile::new().unwrap();
        test_file.write_all("content".as_bytes()).unwrap();

        // call the method under test
        let result = read_file_to_string(test_file.path());
        assert_eq!("content", result.unwrap());
    }
}
