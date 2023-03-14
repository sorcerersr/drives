//!
//! `Drives` A small library to get information about mountable/mounted drives
//! on a Linux system.
//!
//! Uses `/sys/block` to retreive information.
//!

use std::fs::DirEntry;

use fs_wrap::read_file_to_string;

mod fs_wrap;

#[derive(Debug)]
pub enum Error {
    SysDirAccess,
    SysDirEntry,
    SysDirEntryName,
    PathBuild,
    ReadFile,
    FileType,
}

/// A mounted or mountable drive (usually this will be a partition)
#[derive(Debug)]
pub struct Drive {
    /// The name of the block device
    pub device: String,
    /// the name of the partition
    pub partition: String,
    /// is it a fixed device or a removable one like a flash drive or sd card
    pub is_removable: bool,
}

fn read_bool_file(path: &str) -> Result<bool, Error> {
    let content = read_file_to_string(path).map_err(|_err| Error::ReadFile)?;
    Ok("1".eq(&content))
}

fn name_from_direntry(entry: &DirEntry) -> Result<String, Error> {
    entry
        .file_name()
        .into_string()
        .map_err(|_err| Error::SysDirEntryName)
}

fn build_path(dir_entry: &DirEntry, path_to_add: &str) -> Result<String, Error> {
    let mut path = if let Some(path) = dir_entry.path().to_str() {
        path.to_string()
    } else {
        return Err(Error::PathBuild);
    };
    path.push_str(path_to_add);
    Ok(path)
}

fn find_partitions(dir_entry: &DirEntry) -> Result<Vec<String>, Error> {
    let mut partitions = vec![];
    let base_dir_name = name_from_direntry(dir_entry).map_err(|_err| Error::PathBuild)?;

    for entry in fs_wrap::read_dir(dir_entry.path()).map_err(|_err| Error::SysDirAccess)? {
        let entry = entry.map_err(|_err| Error::SysDirEntry)?;
        if entry.file_type().map_err(|_err| Error::FileType)?.is_dir() {
            let dir_name = name_from_direntry(&entry)?;
            if dir_name.starts_with(&base_dir_name) {
                partitions.push(dir_name);
            }
        }
    }
    Ok(partitions)
}

/// Reads /sys/block and its sub-directories to determine and return a list of
/// partitions represented by instances of the Drive struct.
pub fn get_drives() -> Result<Vec<Drive>, Error> {
    let mut drives = vec![];
    for entry in fs_wrap::read_dir("/sys/block").map_err(|_err| Error::SysDirAccess)? {
        let entry = entry.map_err(|_err| Error::SysDirEntry)?;

        let device_name = name_from_direntry(&entry)?;

        let removable_path = build_path(&entry, "/removable")?;
        let removable = read_bool_file(&removable_path)?;

        let partitions = find_partitions(&entry)?;

        if !partitions.is_empty() {
            for partition in partitions {
                let drive = Drive {
                    device: device_name.clone(),
                    partition,
                    is_removable: removable,
                };
                drives.push(drive);
            }
        }
    }
    Ok(drives)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let drives = get_drives().unwrap();
        drives.iter().for_each(|drive| {
            println!("{:?}", drive);
        })
    }
}
