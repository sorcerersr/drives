use std::{
    fs::{self, File, ReadDir},
    io::{self, Read},
    path::Path,
};

pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
    fs::read_dir(path.as_ref())
}

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    let path = Path::new(path);

    let mut file = File::open(path)?;

    let mut result = String::new();
    file.read_to_string(&mut result)?;

    Ok(result)
}
