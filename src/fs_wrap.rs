/*
 * This module wraps the std::fs calls within a trait and an
 * implementing struct to be able to mock it with 'mockall'.
 *
 */
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::{
    fs::{self, File, ReadDir},
    io::{self, Read},
    path::Path,
};

#[cfg_attr(test, automock)]
pub trait FsWrap {
    fn read_dir(&self, path: &str) -> io::Result<ReadDir>;
    fn read_file_to_string(&self, path: &str) -> io::Result<String>;
}

pub struct Filesystem {}

impl FsWrap for Filesystem {
    fn read_dir(&self, path: &str) -> io::Result<ReadDir> {
        fs::read_dir(path)
    }

    fn read_file_to_string(&self, path: &str) -> io::Result<String> {
        let path = Path::new(path);

        let mut file = File::open(path)?;

        let mut result = String::new();
        file.read_to_string(&mut result)?;

        Ok(result)
    }
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {}
    }
}
