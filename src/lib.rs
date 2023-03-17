//!
//! `Drives` is a library to get information about mountable/mounted drives
//! on a Linux system.
//!
//! Uses `/sys/block` to retreive information.
//!

use anyhow::{anyhow, Context, Result};

use std::fs::DirEntry;

mod fs_wrap;

use fs_wrap::{Filesystem, FsWrap};

/// A block device
#[derive(Debug)]
pub struct Device {
    /// The name of the block device
    pub name: String,
    /// list of partitions
    pub partitions: Vec<Partition>,
    /// is it a fixed device or a removable one like a flash drive or sd card
    pub is_removable: bool,
}

/// partition of a device
#[derive(Debug)]
pub struct Partition {
    /// the name of the partitions
    pub name: String,
}

struct Drives {
    filesystem: Box<dyn FsWrap>,
}

impl Drives {
    fn read_bool_file(&self, path: &str) -> Result<bool> {
        let content = self
            .filesystem
            .read_file_to_string(path)
            .with_context(|| format!("Failed to read file from {}", path))?;
        Ok("1".eq(&content))
    }

    fn name_from_direntry(&self, entry: &DirEntry) -> Result<String> {
        if let Ok(result) = entry.file_name().into_string() {
            Ok(result)
        } else {
            return Err(anyhow!("Failed to convert OsString to String"));
        }
    }

    fn build_path(&self, dir_entry: &DirEntry, path_to_add: &str) -> Result<String> {
        let mut path = if let Some(path) = dir_entry.path().to_str() {
            path.to_string()
        } else {
            return Err(anyhow!("Failed to append '{}' to path", path_to_add));
        };
        path.push_str(path_to_add);
        Ok(path)
    }

    fn find_partitions(&self, dir_entry: &DirEntry) -> Result<Vec<Partition>> {
        let mut partitions = vec![];
        let base_dir_name = self.name_from_direntry(dir_entry)?;
        let dir_entry_path = dir_entry
            .path()
            .to_str()
            .with_context(|| format!("failed calling to_str on path {:?}", dir_entry.path()))?
            .to_owned();
        for entry in self
            .filesystem
            .read_dir(&dir_entry_path)
            .with_context(|| format!("Failed to read dir {:#?}", dir_entry.path()))?
        {
            let entry = entry.with_context(|| "Failed to access dir entry")?;
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let dir_name = self.name_from_direntry(&entry)?;
                    if dir_name.starts_with(&base_dir_name) {
                        partitions.push(Partition { name: dir_name });
                    }
                }
            } else {
                return Err(anyhow!("Couldn't get file type for {:?}", entry.path()));
            }
        }
        Ok(partitions)
    }

    fn get_drives(&self) -> Result<Vec<Device>> {
        let mut devices = vec![];
        for entry in self
            .filesystem
            .read_dir("/sys/block")
            .with_context(|| "Failed to access /sys/block")?
        {
            let entry = entry.with_context(|| "Failed to access dir entry")?;

            let device_name = self.name_from_direntry(&entry)?;

            let removable_path = self.build_path(&entry, "/removable")?;
            let removable = self.read_bool_file(&removable_path)?;

            let partitions = self.find_partitions(&entry)?;

            let device = Device {
                name: device_name.clone(),
                partitions,
                is_removable: removable,
            };
            devices.push(device);
        }
        Ok(devices)
    }

    fn new() -> Drives {
        Drives {
            filesystem: Box::new(Filesystem::new()),
        }
    }
}

/// Reads /sys/block and its sub-directories to determine and return drives as a list of
/// devices with partitions
pub fn get_drives() -> Result<Vec<Device>> {
    let drives = Drives::new();
    drives.get_drives()
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

    #[test]
    fn test_read_bool_file_true_case() {
        // mock the filesystem access to return "1" as the content of the
        // file to read
        let mut fsmock = fs_wrap::MockFsWrap::new();
        fsmock
            .expect_read_file_to_string()
            .returning(|_path| Ok("1".to_owned()));

        let drives = Drives {
            filesystem: Box::new(fsmock),
        };
        // call the function under test
        assert!(drives.read_bool_file("testfile").unwrap());
    }

    #[test]
    fn test_read_bool_file_false_case() {
        // mock the filesystem access to return "1" as the content of the
        // file to read
        let mut fsmock = fs_wrap::MockFsWrap::new();
        fsmock
            .expect_read_file_to_string()
            .returning(|_path| Ok("0".to_owned()));

        let drives = Drives {
            filesystem: Box::new(fsmock),
        };
        // call the function under test
        assert!(!drives.read_bool_file("testfile").unwrap());
    }
}
